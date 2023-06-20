import { ParentProps } from "solid-js";

export default function Container(props: ParentProps) {
	return (
		<div class="border dark:border-white border-solid rounded-xl">
			<div class="p-4">{props.children}</div>
		</div>
	);
}
