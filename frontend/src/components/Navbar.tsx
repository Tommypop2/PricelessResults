import { For, onMount } from "solid-js";
import { A } from "solid-start";
import { useNavbarContext } from "~/context/NavbarProvider";
interface NavbarProps {
	options: NavbarOption[];
}
export default function Navbar(props: NavbarProps) {
	const ctx = useNavbarContext();
	let navbar: HTMLDivElement | undefined;
	onMount(() => {
		if (!navbar) return;
		ctx.setNavbar(navbar);
	});
	return (
		<div
			class={`flex flex-row w-full h-full gap-1 m-0 p-0 bg-gradient animate-gradient`}
			ref={navbar}
		>
			<For each={props.options}>
				{(item) => {
					return (
						<div class="h-full flex">
							<A
								href={item.href}
								class="inline-flex items-center m-0 px-5 py-3 rounded-t no-underline"
								activeClass="bg-dark text-white"
								inactiveClass="bg-white text-dark hover:bg-light-600"
								end={true}
							>
								<span class="text-lg">{item.name}</span>
							</A>
						</div>
					);
				}}
			</For>
			{/* <button
				class="ml-auto rounded mr-5 px-5 text-lg border-blue"
				onclick={() => {
					console.log("This works");
				}}
			>
				Login
			</button> */}
		</div>
	);
}
