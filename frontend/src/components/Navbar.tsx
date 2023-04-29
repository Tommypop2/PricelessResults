import { For } from "solid-js";
import { A } from "solid-start";
interface NavbarProps {
	options: NavbarOption[];
}
export default function Navbar(props: NavbarProps) {
	return (
		<div
			class={`flex flex-row w-full h-full gap-1 m-0 p-0 bg-gradient animate-gradient`}
			id="navbar"
		>
			<For each={props.options}>
				{(item, index) => {
					return (
						<div class="h-full flex">
							<A
								href={item.href}
								class="inline-flex items-center m-0 px-5 py-3 rounded-t no-underline"
								activeClass="bg-dark text-white"
								inactiveClass="bg-white text-dark"
								end={true}
							>
								<span class="text-lg">{item.name}</span>
							</A>
						</div>
					);
				}}
			</For>
			<button
				class="ml-auto rounded mr-5 px-5 text-lg border-blue"
				onclick={() => {
					console.log("This works");
				}}
			>
				Login
			</button>
		</div>
	);
}
