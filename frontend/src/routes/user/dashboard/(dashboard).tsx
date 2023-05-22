import { Show, Suspense, createSignal, lazy, onMount } from "solid-js";

const ChartComponent = lazy(() => import("../../../charts"));
function ClientOnly(props: any) {
	const [flag, setFlag] = createSignal(false);

	onMount(() => {
		setFlag(true);
	});

	return <Show when={flag()}>{props.children}</Show>;
}
export default function Dashboard() {
	return (
		<ClientOnly>
			<Suspense>
				<ChartComponent />
			</Suspense>
		</ClientOnly>
	);
}
