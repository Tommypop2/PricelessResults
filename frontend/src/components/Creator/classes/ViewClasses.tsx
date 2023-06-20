import { For, createResource, createSignal } from "solid-js";
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
	const [classes, {mutate}] = createResource(
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
	const [newName, setNewName] = createSignal<HTMLInputElement>();
	return (
		<div class="flex flex-col rounded-xl h-full">
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
				<tbody>
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
				</tbody>
			</table>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					const el = newName();
					if(!el) return;
					const name = el.value;
					let copy = classes();
					if(!copy) return;
					copy?.classes.push({name, members: 0});
					mutate({...copy});
				}}
			>
				<input placeholder="Name" ref={setNewName} autocomplete="off"/>
			</form>
		</div>
	);
}
