import {
	Accessor,
	ParentComponent,
	Setter,
	createContext,
	createSignal,
	useContext,
} from "solid-js";
const NavbarContext = createContext<{
	navbarRef: Accessor<HTMLDivElement | undefined>;
	setNavbarRef: Setter<HTMLDivElement | undefined>;
	options: Accessor<NavbarOption[] | undefined>;
	setOptions: Setter<NavbarOption[] | undefined>;
}>();

export const NavbarProvider: ParentComponent = (props) => {
	const [navbarRef, setNavbarRef] = createSignal<HTMLDivElement | undefined>();
	const [options, setOptions] = createSignal<NavbarOption[]>();
	const value = {
		navbarRef,
		setNavbarRef,
		options,
		setOptions,
	};
	return (
		<NavbarContext.Provider value={value}>
			{props.children}
		</NavbarContext.Provider>
	);
};
export const useNavbarContext = () => useContext(NavbarContext)!;
