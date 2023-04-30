import { createEffect } from "solid-js";
import { useNavbarContext } from "~/context/NavbarProvider";
export default function Home() {
	let header: HTMLHeadingElement | undefined;
	const ctx = useNavbarContext();
	// Sync animation to navbar
	createEffect(() => {
		const navbar = ctx.navbar();
		if (!navbar) return;
		const navbarStartTime = navbar.getAnimations()[0].startTime;
		header!.getAnimations()[0].startTime = navbarStartTime
			? navbarStartTime
			: 0;
	});
	return (
		<main class="w-full h-full">
			<div class="w-full flex flex-row justify-center h-full sm:items-center <sm:text-center">
				<div class="sm:hover:animate-head-shake">
					<h1
						class={`clip-text text-9xl <sm:text-8xl select-none animate-gradient bg-gradient relative z-2`}
						ref={header}
					>
						Priceless Results
					</h1>
				</div>
			</div>
			<div class="absolute right-1 bottom-1 <sm:flex <sm:flex-row <sm:items-center <sm:justify-center <sm:w-full <sm:bottom-16 <sm:right-0">
				<div
					class="absolute w-full h-full z-[-1] animate-pulse animate-duration-[5s] hover:animate-paused"
					style={{
						"background-image":
							"linear-gradient(-120deg, #ee7752, #e73c7e, #23a6d5, #23d5ab)",
						"border-radius": "50%",
						filter: "blur(120px)",
						transform: "translate(-50%,-50%)",
						top: "50%",
						left: "50%",
					}}
				/>
				<img
					src="512x512.webp"
					class="relative hover:animate-pulse w-[400px] h-[400px] <sm:w-[100%] <sm:h-[auto] object-cover"
					draggable={false}
					alt="Logo"
				></img>
			</div>
		</main>
	);
}
