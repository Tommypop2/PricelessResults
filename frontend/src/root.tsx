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
import Navbar from "./components/Navbar";
export default function Root() {
	return (
		<Html lang="en" class="h-full w-full dark">
			<Head>
				<Title>Priceless Results</Title>
				<Meta charset="utf-8" />
				<Meta name="viewport" content="width=device-width, initial-scale=1" />
				<Meta name="description" content="The world's coolest results system" />
				{/* <Link rel="preload" as="font" href="/SourceSansPro-Regular.ttf" crossOrigin="anonymous" /> */}
			</Head>
			<Body class="h-full w-full m-0 p-0 dark:bg-dark dark:text-light">
				<div style={{ height: "5%" }}>
					<Navbar
						options={[
							{ name: "Home", href: "/" },
							{ name: "Help", href: "/help" },
							{ name: "Help more", href: "/ok" },
						]}
					/>
				</div>
				<Suspense>
					<ErrorBoundary>
						<Routes>
							<FileRoutes />
						</Routes>
					</ErrorBoundary>
				</Suspense>
				<Scripts />
			</Body>
		</Html>
	);
}
