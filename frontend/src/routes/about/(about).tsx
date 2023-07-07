import {
	Show,
	Suspense,
	createResource,
	createSignal,
	onMount,
} from "solid-js";
import SolidMarkdown from "solid-markdown";
function RenderMarkdown() {
	const [markdown] = createResource(async () => {
		const res = await fetch(
			"https://raw.githubusercontent.com/Tommypop2/PricelessResults/main/README.md"
		);
		const markdown = await res.text();
		return markdown;
	});
	return (
		<Show when={markdown()}>
			<SolidMarkdown children={markdown()} />
		</Show>
	);
}
export default function About() {
	const [client, setClient] = createSignal(false);
	onMount(() => setClient(true));
	return (
		<div class="p-4">
			<Show when={client()}>
				<RenderMarkdown />
			</Show>
		</div>
	);
}
