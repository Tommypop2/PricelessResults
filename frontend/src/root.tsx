// @refresh reload
import { Suspense } from "solid-js";
import {
	Body,
	ErrorBoundary,
	FileRoutes,
	Head,
	Html,
	Link,
	Meta,
	Routes,
	Scripts,
	Title,
} from "solid-start";
import { Toaster } from "solid-toast";
import "virtual:uno.css";
import "./main.css";
import Navbar from "./components/Navbar/Navbar";
import { NavbarProvider } from "./context/NavbarProvider";
import { ThemeProvider, useThemeContext } from "./context/ThemeProvider";
import { UserProvider } from "./context/UserProvider";
import {
	BsHouse,
	BsQuestionCircle,
	BsInfoCircle,
	BsPieChartFill,
} from "solid-icons/bs";
import { useRegisterSW } from "virtual:pwa-register/solid";
export default function Root() {
	if (globalThis.navigator) {
		useRegisterSW({ immediate: true });
	}
	return (
		<Html lang="en" class="h-full w-full font-[source-sans]">
			<Head>
				<Title>Priceless Results</Title>
				<Meta charset="utf-8" />
				<Meta name="viewport" content="width=device-width, initial-scale=1" />
				<Meta name="description" content="The world's coolest results system" />
				<Meta property="og:image" content="/512x512.webp" />
				<Meta property="og:title" content="The coolest results system" />
				<Link rel="manifest" href="/manifest.webmanifest" />
				{/* Add google gsi globally. This is not ideal, but they don't distribute it via npm yet */}
				<script
					src="https://accounts.google.com/gsi/client"
					async
					defer
					id="googleScript"
				></script>
			</Head>
			<Body class="h-full w-full m-0 p-0 overflow-hidden">
				<ThemeProvider>
					{/* The following code runs executes within the ThemeProvider, so its context can be accessed here */}
					{(() => {
						const themeCtx = useThemeContext();
						return (
							<div class={`h-full w-full ${themeCtx.theme()}`}>
								<div class="h-full w-full dark:bg-dark dark:text-light transition-colors">
									<UserProvider>
										<NavbarProvider>
											<Toaster position="bottom-right" />
											<div class="h-[50px]">
												<Navbar
													options={[
														{ name: "Home", href: "/", icon: BsHouse },
														{
															name: "Help",
															href: "/help",
															icon: BsQuestionCircle,
														},
														{
															name: "About",
															href: "/about",
															icon: BsInfoCircle,
														},
													]}
													loggedInOptions={[
														{
															name: "Dashboard",
															href: "/user/dashboard",
															icon: BsPieChartFill,
														},
													]}
												/>
											</div>
											<div class="h-[calc(100%-50px)]">
												<Suspense>
													<ErrorBoundary>
														<Routes>
															<FileRoutes />
														</Routes>
													</ErrorBoundary>
												</Suspense>
											</div>
											{/* <Toaster position="bottom-right" /> */}
										</NavbarProvider>
									</UserProvider>
								</div>
							</div>
						);
					})()}
				</ThemeProvider>
				<Scripts />
			</Body>
		</Html>
	);
}
