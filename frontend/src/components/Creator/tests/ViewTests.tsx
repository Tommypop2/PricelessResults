import { ImBin } from "solid-icons/im";
import { For, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import AddTest from "~/components/AddTest/AddTest";
import { Test } from "~/components/User/tests/UserTests";
import { GetCreatedResult, TestsView } from "./TestsView";
import { createTest, deleteTest } from "~/helpers/tests/tests";
interface ViewTestsProps {
	session_id?: string;
}
export function ViewTests(props: ViewTestsProps) {
	const [tests, { mutate }] = createResource(
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
	const testUpdater = (newTests: Test[]) => {
		mutate({ ...tests()!, tests: newTests });
	};
	return (
		<>
			<div class="relative h-full mx-2 text-left">
				<TestsView
					tests={tests()!}
					updateTests={testUpdater}
					session_id={props.session_id}
					onTestClicked={(test) => {}}
					onButtonClicked={async (item) => {
						if (
							!confirm(
								"Are you sure you want to delete this test? This action is permanent and cannot be undone."
							)
						)
							return;
						const res = await deleteTest(item.id, props.session_id!);
						testUpdater(tests()!.tests.filter((test) => test.id !== item.id));
					}}
					buttonIcon={ImBin}
					buttonTitle="Delete Test"
				/>
				<div class="absolute bottom-0 p-b-2 w-full">
					<AddTest
						onAddTest={async (name, max_score) => {
							if (!name || !max_score) return false;
							const response = await createTest(
								{ name, max_score },
								props.session_id!
							);
							if (response?.success) {
								const id = response.test?.id;
								testUpdater(
									tests()!.tests.concat({
										name,
										max_score,
										id: id!,
										assignees: 0,
									})
								);
								toast.success("Test created");
								return true;
							}
							toast.error("Failed to create test");
							return false;
						}}
					/>
				</div>
			</div>
		</>
	);
}
