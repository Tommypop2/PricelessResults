import { For, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import { createClass, deleteClass } from "~/helpers/classes/class";
import { VsCopy } from "solid-icons/vs";
import { ImBin } from "solid-icons/im";
type ClassRecord = { name: string; members: number; id: string };
type GetMembershipResult = {
	success: boolean;
	classes: ClassRecord[];
	error?: string;
};
type ClasesViewProps = {
	session_id?: string;
};
export default function ViewClasses(props: ClasesViewProps) {
	const [classes, { mutate }] = createResource(
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
		<div class="flex flex-col rounded-xl h-full p-0 m-0 mx-4">
			<table>
				<thead>
					<tr>
						<th class="w-[100%]">
							<h2>My Classes</h2>
						</th>
						<th>
							<h2>Members</h2>
						</th>
						<th></th>
					</tr>
				</thead>
				<tbody>
					<For each={classes()?.classes}>
						{(item, i) => {
							return (
								<tr class="border border-solid border-white rounded-xl py-20 text-xl">
									<td>{item.name}</td>
									<td>{item.members}</td>
									<td>
										<button
											title="Copy join link"
											class="rounded bg-transparent border-none active:animate-jello animate-duration-75"
											onClick={() => {
												const id = item.id.split(":")[1];
												if (!id) return;
												const origin = window.location.origin;
												navigator.clipboard.writeText(
													`${origin}/join/class?class_id=${id}`
												);
												toast.success("Copied join link");
											}}
										>
											<VsCopy size={30} />
										</button>
									</td>
									<td>
										<button
											class="rounded bg-transparent border-none active:animate-jello animate-duration-75"
											onClick={async () => {
												const id = item.id.split(":")[1];
												if (!id) return;
												if (
													!confirm(
														"Are you sure you want to delete this class? This action cannot be undone."
													)
												) {
													return;
												}
												const res = await deleteClass(id, props.session_id);
												if (!res) {
													toast.error("Result unknown");
													return;
												}
												if (!res.success) {
													toast.error(res.error);
													return;
												}
												let copy = classes();
												if (!copy) return;
												copy.classes = copy.classes.filter(
													(c) => c.id !== item.id
												);
												mutate({ ...copy });
												toast.success("Class deleted");
											}}
										>
											<ImBin size={30} />
										</button>
									</td>
								</tr>
							);
						}}
					</For>
				</tbody>
			</table>
			<form
				onsubmit={async (e) => {
					e.preventDefault();
					const el = newName();
					if (!el) return;
					const name = el.value;
					const res = await createClass({ name }, props.session_id);
					if (!res?.success || !res?.class) {
						toast.error("Failed to create class");
						return;
					}
					el.value = "";
					toast.success("Class created");
					let copy = classes();
					if (!copy) return;
					copy?.classes.push({ name, members: 0, id: res.class.id });
					mutate({ ...copy });
				}}
			>
				<input placeholder="Name" ref={setNewName} autocomplete="off" />
			</form>
		</div>
	);
}
