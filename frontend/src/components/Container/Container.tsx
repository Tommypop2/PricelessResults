import { ParentProps } from "solid-js";

export default function Container(props: ParentProps) {
	return (
		<div class="border dark:border-white border-solid rounded-xl">
			<div class="h-full">{props.children}</div>
		</div>
	);
}
