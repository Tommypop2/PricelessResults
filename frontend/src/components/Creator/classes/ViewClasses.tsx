import { For, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import { createClass, deleteClass } from "~/helpers/classes/class";
import { VsCopy } from "solid-icons/vs";
import { ImBin } from "solid-icons/im";
import AddClass from "~/components/AddClass/AddClass";
import { useNavigate } from "solid-start";
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
			return resJson;
		}
	);
	const [newName, setNewName] = createSignal<HTMLInputElement>();
	const navigate = useNavigate();
	return (
		<div class="relative rounded-xl h-full p-0 m-0 mx-4 min-h-50 text-left">
			<table class="border-collapse">
				<thead>
					<tr>
						<th class="w-full">
							<h2>My Classes</h2>
						</th>
						<th>
							<h2>Members</h2>
						</th>
						<th class="min-w-[44px]"></th>
						<th class="min-w-[44px]"></th>
					</tr>
				</thead>
				<tbody>
					<For each={classes()?.classes}>
						{(item, i) => {
							return (
								<tr
									class="text-xl"
									onClick={() => {
										const id = item.id.split(":")[1];
										if (!id) return;
										navigate(`/creator/class/${id}`);
									}}
								>
									<td>{item.name}</td>
									<td>{item.members}</td>
									<td>
										<button
											title="Copy join link"
											class="rounded bg-transparent border-none active:animate-jello animate-duration-75"
											onClick={(e) => {
												e.stopPropagation();
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
											onClick={async (e) => {
												e.stopPropagation();
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
			<div class="absolute bottom-0 p-b-2 w-full">
				<AddClass
					onAddClass={async (name) => {
						if (!name) return false;
						const res = await createClass({ name }, props.session_id);
						if (!res?.success || !res?.class) {
							toast.error("Failed to create class");
							return false;
						}
						toast.success("Class created");
						let copy = classes();
						if (!copy) return false;
						copy?.classes.push({ name, members: 0, id: res.class.id });
						mutate({ ...copy });
						return true;
					}}
				/>
			</div>
		</div>
	);
}
