import {
	ParentComponent,
	Resource,
	Setter,
	createContext,
	createResource,
	useContext,
} from "solid-js";
import { isServer } from "solid-js/web";
import { parseCookie } from "solid-start";
import { useRequest } from "solid-start/server";
export type User = {
	user_id: string;
	username: string;
	email: string;
	picture: string;
	session_id: string;
	admin: boolean;
};
type LoginResult = {
	session_id?: string;
	error?: string;
	user?: Partial<User>;
};
type UserResult = {
	success: boolean;
	error?: string;
	user?: Partial<User>;
};
const UserContext = createContext<{
	user: Resource<User | undefined>;
	login: (id_token: string) => Promise<string | undefined>;
	logout: (session_id?: string | undefined) => Promise<void>;
	invalidateSession: (session_id: string) => Promise<void>;
	mutate: Setter<User | undefined>;
	refetch: (
		info?: unknown
	) => User | Promise<User | undefined> | null | undefined;
}>();
export const UserProvider: ParentComponent = (props) => {
	// Retrieves user from the backend
	const [user, { mutate, refetch }] = createResource(async () => {
		const event = useRequest();
		const sessionId = parseCookie(
			isServer ? event.request.headers.get("cookie") ?? "" : document.cookie
		)["session_id"];
		if (!sessionId) return;
		let res: Response;
		try {
			res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/user/user?session_id=${sessionId}`
			);
		} catch {
			return;
		}
		const userJson: UserResult = await res.json();
		if (userJson.success == false) return;
		let userData = userJson["user"];
		userData!.session_id = sessionId;
		return userData as User;
	});
	const invalidateSession = async (session_id: string) => {
		await fetch(
			`${import.meta.env.VITE_SERVER_URI}/user/logout?session_id=${session_id}`
		);
	};
	const value = {
		user,
		login: async (id_token: string) => {
			const res: LoginResult = await (
				await fetch(`${import.meta.env.VITE_SERVER_URI}/user/login`, {
					credentials: "include",
					method: "post",
					headers: {
						"Content-Type": "application/json",
					},
					body: JSON.stringify({ id_token: id_token }),
				})
			).json();
			let newUser = res.user;
			if (!newUser) {
				return res.error;
			}
			document.cookie = `session_id=${res.session_id}; max-age=${
				1 * 24 * 60 * 60
			}; SameSite=Strict; Secure; path=/`;
			newUser["session_id"] = res.session_id;
			// By this time the user is no longer "partial" as it now has the `session_id` property
			mutate(newUser as User);
		},
		logout: async (session_id = user()?.session_id) => {
			if (!session_id) return;
			// Remove cookie
			document.cookie =
				"session_id=;max-age=0; SameSite=Strict; Secure; path=/";
			// Remove user from context
			await invalidateSession(session_id);
			mutate();
		},
		invalidateSession,
		mutate,
		refetch,
	};
	return (
		<UserContext.Provider value={value}>{props.children}</UserContext.Provider>
	);
};
export const useUserContext = () => useContext(UserContext)!;
