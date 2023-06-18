import { For, createResource } from "solid-js";
export type Class = { name: string };
type ClassMembership = { class: Class };
type GetMembershipResult = {
	success: boolean;
	memberships: ClassMembership[];
	error?: string;
};
type ClasesViewProps = {
	session_id?: string;
};
export default function ClassesView(props: ClasesViewProps) {
	const [classes] = createResource(
		() => props.session_id,
		async (id) => {
			if (!id) return { success: false, memberships: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/class/get_joined?session_id=${id}`
			);
			const resJson = (await res.json()) as GetMembershipResult;
			return resJson;
		}
	);
	return (
		<div class="flex flex-col">
			<For each={classes()?.memberships}>
				{(item, i) => {
					return <div>{item.class.name}</div>;
				}}
			</For>
		</div>
	);
}
