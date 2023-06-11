import solid from "solid-start/vite";
import { defineConfig } from "vite";
import UnoCss from "unocss/vite";
import vercel from "solid-start-vercel";
import { VitePWA } from "vite-plugin-pwa";
export default defineConfig({
	plugins: [
		solid({ adapter: vercel({ edge: true }) }),
		UnoCss(),
		VitePWA({
			registerType: "autoUpdate",
			devOptions: { enabled: true, type: "module" },
			workbox: {
				// Shouldn't really precache much because the user's storage space could be limited. However, it's worth precaching some of the javascript and css
				globPatterns: ["**/*.{js,css}"],
				runtimeCaching: [
					{
						// Tries to go through the network first, and, if that fails, it resorts to the cache. This means that possibly outdated data is the last resort
						handler: "NetworkFirst",
						urlPattern: (url) => {
							// We basically want to cache everything by default. This should be done at runtime instead of precaching because of SSR
							return true;
						},
					},
				],

				navigateFallback: null,
			},
			base: "/",
			manifest: {
				name: "Priceless Results",
				short_name: "Priceless Results",
				description: "Very cool results system",
				start_url: "/",
				icons: [
					{
						src: "/android-chrome-192x192.png",
						sizes: "192x192",
						type: "image/png",
					},
					{
						src: "/android-chrome-512x512.png",
						sizes: "512x512",
						type: "image/png",
					},
				],
				theme_color: "#ffffff",
				background_color: "#ffffff",
				display: "standalone",
			},
		}),
	],
});
