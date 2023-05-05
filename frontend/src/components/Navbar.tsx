import { For, Show, Suspense, onMount } from "solid-js";
import { A } from "solid-start";
import { useNavbarContext } from "~/context/NavbarProvider";
import { BsMoon, BsSun } from "solid-icons/bs";
import { useThemeContext } from "~/context/ThemeProvider";
import { useUserContext } from "~/context/UserProvider";
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
async function waitForGoogleLoad() {
	// Return if script is already loaded
	if (typeof google != "undefined") return;
	// Else, wait for the script tag to load
	const element = document.getElementById("googleScript")!;
	return await new Promise<void>((res, rej) => {
		element.onload = () => {
			res();
		};
	});
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

	const userCtx = useUserContext();
	onMount(async () => {
		if (!loginWithGoogleButton) return;
		await waitForGoogleLoad();
		google.accounts.id.initialize({
			client_id:
				"840942651861-i3g0m9jvt8j0js61ik1i54at9p8m7v9s.apps.googleusercontent.com",
			nonce: generateRandomString(64),
			callback: async (response: google.accounts.id.CredentialResponse) => {
				// None of this is typed yet
				const res = await (
					await fetch(`${import.meta.env.VITE_SERVER_URI}/user/login`, {
						credentials: "include",
						method: "post",
						headers: {
							"Content-Type": "application/json",
						},
						body: JSON.stringify({ id_token: response.credential }),
					})
				).json();
				document.cookie = `session_id=${res.session_id}; max-age=${
					1 * 24 * 60 * 60
				}`;
				const user = res.user;
				userCtx.mutate(user);
			},
		});
		google.accounts.id.renderButton(loginWithGoogleButton, {
			theme: "outline",
			size: "large",
			shape: "square",
			type: "icon",
		});
		google.accounts.id.prompt();
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
				<div class="flex justify-center flex-col h-full w-[40px] mr-4">
					<Suspense>
						<Show
							when={!userCtx.user()}
							fallback={
								<img src={userCtx.user()?.picture} class="rounded-50"></img>
							}
						>
							<div ref={loginWithGoogleButton}></div>
						</Show>
					</Suspense>
				</div>
			</div>
		</div>
	);
}
