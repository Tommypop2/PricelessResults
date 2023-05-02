import { For, Show, onMount } from "solid-js";
import { A } from "solid-start";
import { useNavbarContext } from "~/context/NavbarProvider";
import { BsMoon, BsSun } from "solid-icons/bs";
import { useThemeContext } from "~/context/ThemeProvider";
interface NavbarProps {
	options?: NavbarOption[];
}
const characters =
	"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
function generateRandomString(length: number) {
	let result = "";
	for (let i = 0; i < length; i++) {
		result += characters.charAt(Math.floor(Math.random() * characters.length));
	}
	return result;
}
export default function Navbar(props: NavbarProps) {
	const navCtx = useNavbarContext();
	let navbar: HTMLDivElement | undefined;
	let loginWithGoogleButton: HTMLDivElement | undefined;
	if (props.options) navCtx.setOptions(props.options);
	onMount(() => {
		if (!navbar) return;
		navCtx.setNavbarRef(navbar);
	});
	const themeCtx = useThemeContext();
	const isDark = () => themeCtx.theme() === "dark";
	onMount(() => {
		if (!loginWithGoogleButton) return;
		google.accounts.id.initialize({
			client_id:
				"840942651861-i3g0m9jvt8j0js61ik1i54at9p8m7v9s.apps.googleusercontent.com",
			callback: (response: any) => {
				console.log(response);
			},
		});
		google.accounts.id.renderButton(loginWithGoogleButton, {
			theme: "outline",
			size: "large",
			shape: "square",
			type: "icon",
		});
	});
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
								activeClass={
									isDark() ? "bg-dark text-white" : "bg-white text-dark"
								}
								inactiveClass={
									isDark()
										? "bg-white text-dark hover:bg-light-600"
										: "bg-dark text-white hover:bg-dark-3"
								}
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

				<div
					ref={loginWithGoogleButton}
					class="flex justify-center flex-col"
				></div>
				{/* <button
					class="rounded mr-5 px-5 text-lg border-blue"
					onclick={() => {
						console.log("This works");
					}}
				>
					Login
				</button> */}
			</div>
		</div>
	);
}
