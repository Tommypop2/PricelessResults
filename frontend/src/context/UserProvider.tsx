import {
	Accessor,
	ParentComponent,
	Resource,
	Setter,
	createContext,
	createMemo,
	createResource,
	createSignal,
	useContext,
} from "solid-js";
import { isServer } from "solid-js/web";
import { parseCookie } from "solid-start";
import { useRequest } from "solid-start/server";
type User = {
	userName: string;
	userId: string;
};
const UserContext = createContext<{
	user: Resource<User | undefined>;
	mutate: Setter<User | undefined>;
	refetch: (
		info?: unknown
	) => User | Promise<User | undefined> | null | undefined;
}>();
export const UserProvider: ParentComponent = (props) => {
	const [user, { mutate, refetch }] = createResource(async () => {
		const event = useRequest();
		const sessionId = parseCookie(
			isServer ? event.request.headers.get("cookie") ?? "" : document.cookie
		)["session_id"];
		if (!sessionId) return;
		const res = await fetch(
			`${import.meta.env.VITE_SERVER_URI}/user/getuser?session_id=${sessionId}`
		);
		const userJson = await res.json();
		if (userJson["success"] == false) {
			return;
		}
		return userJson as User;
	});
	const value = {
		user,
		mutate,
		refetch,
	};
	return (
		<UserContext.Provider value={value}>{props.children}</UserContext.Provider>
	);
};
export const useUserContext = () => useContext(UserContext)!;
