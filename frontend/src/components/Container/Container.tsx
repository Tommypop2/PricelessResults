import { ParentProps } from "solid-js";
interface ContainerProps extends ParentProps {
	class?: string;
}
export default function Container(props: ContainerProps) {
	return (
		<div
			class={`border dark:border-white border-solid rounded-xl ${
				props.class ?? ""
			}`}
		>
			<div class="h-full">{props.children}</div>
		</div>
	);
}
