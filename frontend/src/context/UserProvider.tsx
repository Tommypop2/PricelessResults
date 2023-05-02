import {
	Accessor,
	ParentComponent,
	Setter,
	createContext,
	createSignal,
	useContext,
} from "solid-js";
type User = {
	userName: string;
	userId: string;
};
const UserContext = createContext<{
	user: Accessor<User | undefined>;
	setUser: Setter<User | undefined>;
}>();

export const UserProvider: ParentComponent = (props) => {
	const [user, setUser] = createSignal<User>();
	const value = {
		user,
		setUser,
	};
	return (
		<UserContext.Provider value={value}>{props.children}</UserContext.Provider>
	);
};
export const useUserContext = () => useContext(UserContext)!;
