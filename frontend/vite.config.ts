import solid from "solid-start/vite";
import { defineConfig } from "vite";
import UnoCss from "unocss/vite";
export default defineConfig({
	plugins: [solid(), UnoCss()],
});
