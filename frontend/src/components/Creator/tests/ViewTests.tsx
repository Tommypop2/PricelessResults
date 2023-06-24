import { ImBin } from "solid-icons/im";
import { For, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import AddTest from "~/components/AddTest/AddTest";
import { Test } from "~/components/User/tests/UserTests";
import { createTest } from "~/helpers/tests/tests";
type GetCreatedResult = {
	success: boolean;
	error?: string;
	tests: Test[];
};
interface ViewTestsProps {
	session_id?: string;
}
export function ViewTests(props: ViewTestsProps) {
	const [tests] = createResource(
		() => props.session_id,
		async (id) => {
			if (!id) return { success: false, tests: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/tests/get_created?session_id=${id}`
			);
			const resJson = (await res.json()) as GetCreatedResult;
			return resJson;
		}
	);
	return (
		<div class="relative rounded-xl h-full mx-2 m-0">
			<h2 class="p-0 m-0">My Tests</h2>
			<For each={tests()?.tests}>
				{(item, i) => {
					// This sucks, but it's ok for prototyping
					return (
						<div>
							{item.name}
							{"  "}
							{item.max_score}
						</div>
					);
				}}
			</For>
			<div class="absolute bottom-0 p-b-2 w-full">
				<AddTest
					onAddTest={async (name, max_score) => {
						if (!name || !max_score) return false;
						if (await createTest({ name, max_score }, props.session_id!)) {
							toast.success("Test created");
							return true;
						}
						toast.error("Failed to create test");
						return false;
					}}
				/>
			</div>
		</div>
	);
}
