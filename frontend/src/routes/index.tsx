import { onMount } from "solid-js";
export default function Home() {
	let header: HTMLHeadingElement | undefined;
	// Sync animation to navbar
	onMount(() => {
		const navbar = document.getElementById("navbar")!;
		const navbarStartTime = navbar.getAnimations()[0].startTime;
		header!.getAnimations()[0].startTime = navbarStartTime;
	});
	return (
		<main class="w-full h-[95%]">
			<div class="w-full flex flex-row justify-center h-full items-center">
				<div>
					<h1
						class={`clip-text text-9xl select-none animate-gradient bg-gradient`}
						ref={header}
					>
						Priceless Results
					</h1>
				</div>
			</div>
			<div class="absolute right-1 bottom-1">
				<div
					class="absolute w-full h-full z-[-1]"
					style={{
						opacity: "0.8",
						"background-image":
							"linear-gradient(-120deg, #ee7752, #e73c7e, #23a6d5, #23d5ab)",
						"border-radius": "50%",
						filter: "blur(120px)",
						transform: "translate(-50%,-50%)",
						top: "50%",
						left: "50%",
					}}
				/>
				<img src="android-chrome-512x512.png" class="relative"></img>
			</div>
		</main>
	);
}
