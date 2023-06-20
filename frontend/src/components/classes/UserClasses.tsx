import { For, createResource } from "solid-js";
export type Class = { name: string };
type ClassMembership = { class: Class, members: number };
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
			console.log(resJson);
			return resJson;
		}
	);
	return (
		<div class="flex flex-col rounded-xl">
			<table>
				<thead>
					<tr>
						<th class="w-[100%]">
							<h2>My Classes</h2>
						</th>
						<th>
							<h2>Members</h2>
						</th>
					</tr>
				</thead>
				<tbody></tbody>
				<For each={classes()?.memberships}>
					{(item, i) => {
						return (
							<tr>
								<td>{item.class.name}</td>
								<td>{item.members}</td>
							</tr>
						);
					}}
				</For>
			</table>
		</div>
	);
}
