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
import "virtual:uno.css";
import "./main.css";
import Navbar from "./components/Navbar/Navbar";
import { NavbarProvider } from "./context/NavbarProvider";
import { ThemeProvider, useThemeContext } from "./context/ThemeProvider";
import { UserProvider } from "./context/UserProvider";
export default function Root() {
	return (
		<Html lang="en" class="h-full w-full font-[source-sans]">
			<Head>
				<Title>Priceless Results</Title>
				<Meta charset="utf-8" />
				<Meta name="viewport" content="width=device-width, initial-scale=1" />
				<Meta name="description" content="The world's coolest results system" />
				<script
					src="https://accounts.google.com/gsi/client"
					async
					defer
					id="googleScript"
				></script>

				{/* <Link rel="preload" as="font" href="/SourceSansPro-Regular.ttf" crossOrigin="anonymous" /> */}
			</Head>
			<Body class="h-full w-full m-0 p-0 overflow-hidden">
				<ThemeProvider>
					{(() => {
						const themeCtx = useThemeContext();
						return (
							<div class={`h-full w-full ${themeCtx.theme()}`}>
								<div class="h-full w-full dark:bg-dark dark:text-light transition-colors">
									<UserProvider>
										<NavbarProvider>
											<div class="h-[50px]">
												<Navbar
													options={[
														{ name: "Home", href: "/" },
														{ name: "Help", href: "/help" },
														{ name: "Help more", href: "/ok" },
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
