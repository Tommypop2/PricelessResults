import {
	Accessor,
	ParentComponent,
	Setter,
	createContext,
	createEffect,
	createSignal,
	on,
	useContext,
} from "solid-js";
import { isServer } from "solid-js/web";
import { parseCookie } from "solid-start";
import { useRequest } from "solid-start/server";
type Theme = "light" | "dark";
const ThemeContext = createContext<{
	theme: Accessor<Theme>;
	setTheme: Setter<Theme>;
}>();

export const ThemeProvider: ParentComponent = (props) => {
	const event = useRequest();
	const userTheme = parseCookie(
		isServer ? event.request.headers.get("cookie") ?? "" : document.cookie
	)["theme"] as Theme;
	const [theme, setTheme] = createSignal<Theme>((userTheme) ?? "dark");
	const value = {
		theme,
		setTheme,
	};
	createEffect(
		on(
			theme,
			(newTheme) => {
				document.cookie = `theme=${newTheme};max-age=86400;`;
			},
			{ defer: true }
		)
	);
	return (
		<ThemeContext.Provider value={value}>
			{props.children}
		</ThemeContext.Provider>
	);
};
export const useThemeContext = () => useContext(ThemeContext)!;
