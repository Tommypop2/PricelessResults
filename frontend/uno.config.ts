import { defineConfig } from "unocss";
import presetWind from "@unocss/preset-wind";

export default defineConfig({
	theme: {
		animation: {
			keyframes: {
				gradient: `
				{ 0% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
		100% {
			background-position: 0% 50%;
		}}`,
			},
			durations: { gradient: "15s" },
			timingFns: { gradient: "ease" },
			counts: { gradient: "infinite" },
		},
	},
	rules: [
		[
			"clip-text",
			{
				"-webkit-background-clip": "text",
				"-webkit-text-fill-color": "transparent",
				"-moz-background-clip": "text",
				"-moz-text-fill-color": "transparent",
			},
		],
		[
			"bg-gradient",
			{
				"background-image":
					"linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab)",
				"background-size": "400%",
				"background-repeat": "repeat",
			},
		],
	],
	presets: [presetWind()],
});
