import {
	Accessor,
	ParentComponent,
	Setter,
	createContext,
	createSignal,
	useContext,
} from "solid-js";

const NavbarContext = createContext<{
	navbar: Accessor<HTMLDivElement | undefined>;
	setNavbar: Setter<HTMLDivElement | undefined>;
}>();

export const NavbarProvider: ParentComponent = (props) => {
	const [navbar, setNavbar] = createSignal<HTMLDivElement | undefined>();
	const value = {
		navbar,
		setNavbar,
	};
	return (
		<NavbarContext.Provider value={value}>
			{props.children}
		</NavbarContext.Provider>
	);
};
export const useNavbarContext = () => useContext(NavbarContext)!;
