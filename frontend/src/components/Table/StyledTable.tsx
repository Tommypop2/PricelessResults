import { ParentProps } from "solid-js";

export function Tr(props: ParentProps) {
	return <tr>{props.children}</tr>;
}
export function Th(props: ParentProps) {
	return <th>{props.children}</th>;
}
export function Td(props: ParentProps) {
	return <td>{props.children}</td>;
}
