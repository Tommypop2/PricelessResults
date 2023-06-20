import { For, createResource } from "solid-js";
type ClassRecord = { name: string; members: number };
type GetMembershipResult = {
	success: boolean;
	classes: ClassRecord[];
	error?: string;
};
type ClasesViewProps = {
	session_id?: string;
};
export default function ClassesView(props: ClasesViewProps) {
	const [classes] = createResource(
		() => props.session_id,
		async (id) => {
			if (!id) return { success: false, classes: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/class/get_created?session_id=${id}`
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
				<For each={classes()?.classes}>
					{(item, i) => {
						return (
							<tr>
								<td>{item.name}</td>
								<td>{item.members}</td>
							</tr>
						);
					}}
				</For>
			</table>
		</div>
	);
}
