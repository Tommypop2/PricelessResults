import { For, Show, onMount } from "solid-js";
import { A } from "solid-start";
import { useNavbarContext } from "~/context/NavbarProvider";
import * as oauth from "oauth4webapi";
import { BsMoon, BsSun } from "solid-icons/bs";
import { useThemeContext } from "~/context/ThemeProvider";
interface NavbarProps {
	options?: NavbarOption[];
}
export default function Navbar(props: NavbarProps) {
	const navCtx = useNavbarContext();
	let navbar: HTMLDivElement | undefined;
	if (props.options) navCtx.setOptions(props.options);
	onMount(() => {
		if (!navbar) return;
		navCtx.setNavbarRef(navbar);
	});

	const themeCtx = useThemeContext();
	return (
		<div
			class={`flex flex-row w-full h-full gap-1 m-0 p-0 bg-gradient animate-gradient`}
			ref={navbar}
		>
			<For each={navCtx.options()}>
				{(item) => {
					return (
						<div class="h-full flex">
							<A
								href={item.href}
								class="inline-flex items-center m-0 px-5 py-3 rounded-t-1 no-underline"
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
			<div class="flex flex-row ml-auto gap-2">
				<button
					title="yes"
					class="rounded opacity-80 hover:opacity-100 md:px-1 bg-transparent border-none active:animate-tada animate-duration-75"
					onClick={() =>
						themeCtx.setTheme((prev) => (prev === "light" ? "dark" : "light"))
					}
				>
					<Show
						when={themeCtx.theme() == "dark"}
						fallback={<BsMoon size={30} />}
					>
						<BsSun size={30} />
					</Show>
				</button>
				<button
					class="rounded mr-5 px-5 text-lg border-blue"
					onclick={() => {
						const code_verifier = oauth.generateRandomCodeVerifier();
						const code_challenge =
							oauth.calculatePKCECodeChallenge(code_verifier);
						const client_id = import.meta.env.VITE_GOOGLE_CLIENT_ID as string;
						const redirect_uri = "http://localhost:5173";
						const url = new URL("https://accounts.google.com/o/oauth2/v2/auth");

						console.log("This works");
					}}
				>
					Login
				</button>
			</div>
		</div>
	);
}
