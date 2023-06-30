import { createResource, createSignal } from "solid-js";
import { useNavigate, useParams } from "solid-start";
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
import { IoRemoveCircleSharp } from "solid-icons/io";
export default function ClassView() {
	const params = useParams();
	const userCtx = useUserContext();
	const navigate = useNavigate();
	const session_id = () => userCtx.user()?.session_id;
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
	const [allTests, { mutate }] = createResource(
		() => [session_id(), searchQuery()],
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
			<div class="grid grid-cols-4 gap-2">
				<Container class="min-h-100">
					<div class="relative h-full mx-2 text-left">
						<TestsView
							session_id={session_id()}
							tests={allTests.latest!}
							updateTests={() => {}}
							onTestClicked={(test) => {
								const testId = test.id.split(":")[1];
								const classId = data()?.class.id.split(":")[1];
								if (!testId || !classId) return;
								navigate(
									`/creator/test/add_scores?class_id=${classId}&test_id=${testId}`
								);
							}}
							onButtonClicked={async (test) => {
								const id = test.id.split(":")[1];
								if (!id) return;
								const res = await assignTest(id, params.id, session_id()!);
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
				<Container>
					<TestsView
						session_id={session_id()}
						tests={allTests.latest!}
						updateTests={() => {}}
						onTestClicked={() => {}}
						buttonIcon={IoRemoveCircleSharp}
						buttonTitle="Assign To Class"
						onButtonClicked={() => {}}
					/>
				</Container>
			</div>
		</>
	);
}
