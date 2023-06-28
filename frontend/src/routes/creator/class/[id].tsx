import { createResource, createSignal } from "solid-js";
import { useParams } from "solid-start";
import Container from "~/components/Container/Container";
import Search from "~/components/Creator/tests/Search";
import {
	GetCreatedResult,
	TestsView,
} from "~/components/Creator/tests/TestsView";
import { useUserContext } from "~/context/UserProvider";
import { FaSolidShare } from "solid-icons/fa";
import { assignTest } from "~/helpers/tests/tests";
import toast from "solid-toast";
import { Test } from "~/components/User/tests/UserTests";

export default function ClassView() {
	const params = useParams();
	const userCtx = useUserContext();
	// Get class data
	const [data] = createResource(
		() => [params.id, userCtx.user()?.session_id],
		async ([class_id, session_id]) => {
			const data = await fetch(
				`${
					import.meta.env.VITE_SERVER_URI
				}/class/get_single?id=${class_id}&session_id=${session_id}`
			);
			const json = await data.json();
			return json;
		}
	);
	const [searchQuery, setSearchQuery] = createSignal("");
	const [tests, { mutate }] = createResource(
		() => [userCtx.user()?.session_id, searchQuery()],
		async ([session_id, query]) => {
			const res = await fetch(
				`${
					import.meta.env.VITE_SERVER_URI
				}/tests/fuzzy_find?session_id=${session_id}&search=${query}`
			);
			const resJson = await res.json();
			return resJson as GetCreatedResult;
		}
	);
	return (
		<>
			<div class="flex flex-row justify-center">
				<h1 class="text-6xl m-3">{data()?.class.name}</h1>
			</div>
			<div class="grid grid-cols-4">
				<Container class="min-h-100">
					<div class="relative h-full mx-2 text-left">
						<TestsView
							session_id={userCtx.user()?.session_id}
							tests={tests.latest!}
							updateTests={() => {}}
							onTestClicked={() => {}}
							onButtonClicked={async (test) => {
								const id = test.id.split(":")[1];
								if (!id) return;
								const res = await assignTest(
									id,
									params.id,
									userCtx.user()?.session_id!
								);
								const newTestData = res.test as Test;
								mutate((prev) => {
									if (!prev) return;
									return {
										...prev,
										tests: prev.tests.map((test) => {
											if (test.id === newTestData.id) {
												return newTestData;
											}
											return test;
										}),
									};
								});
								if (res.success) {
									toast.success("Test assigned successfully");
								}
							}}
							buttonIcon={FaSolidShare}
							buttonTitle="Assign To Class"
						/>
						<div class="absolute bottom-0 p-b-2 w-full">
							<Search onChange={setSearchQuery} />
						</div>
					</div>
				</Container>
			</div>
		</>
	);
}
