import {
	Accessor,
	ParentComponent,
	Setter,
	createContext,
	createEffect,
	createSignal,
	on,
	onMount,
	useContext,
} from "solid-js";
import { isServer } from "solid-js/web";
import { parseCookie } from "solid-start";
import { useRequest } from "solid-start/server";
type Theme = "light" | "dark";
const ThemeContext = createContext<{
	theme: Accessor<Theme>;
	setTheme: (value: Theme) => Theme;
}>();

export const ThemeProvider: ParentComponent = (props) => {
	// Gets the user's theme isomorphically (either from document.cookie on the client, or request.headers.get("cookie") on the server)
	const event = useRequest();
	const userTheme = parseCookie(
		isServer ? event.request.headers.get("cookie") ?? "" : document.cookie
	)["theme"] as Theme;
	const [theme, setTheme] = createSignal<Theme>(userTheme ?? "dark");
	let channel: BroadcastChannel;
	onMount(() => {
		channel = new BroadcastChannel("theme_channel");
		channel.onmessage = (e) => {
			const newTheme = e.data as Theme;
			setTheme(newTheme);
		};
	});
	const updateTheme = (newTheme: Theme) => {
		setTheme(newTheme);
		const maxAge = 365 * 24 * 60 * 60;
		document.cookie = `theme=${newTheme};max-age=${maxAge};path=/;`;
		channel.postMessage(newTheme);
		return newTheme;
	};
	const value = {
		theme,
		setTheme: updateTheme,
	};
	return (
		<ThemeContext.Provider value={value}>
			{props.children}
		</ThemeContext.Provider>
	);
};
export const useThemeContext = () => {
	const ctx = useContext(ThemeContext);
	if (!ctx) {
		throw Error("Theme context not found");
	}
	return ctx;
};
