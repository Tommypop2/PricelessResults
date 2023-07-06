import { For, createMemo, createResource } from "solid-js";
import { useNavigate } from "solid-start";
import { TestsView } from "~/components/Creator/tests/TestsView";
import { User } from "~/context/UserProvider";
import { Score, ScoresResult } from "~/helpers/scores/scores";
export type Test = {
	name: string;
	max_score: number;
	id: string;
	assignees: number;
	score?: number;
};
export type TestMembership = { test: Test; user?: User; score?: Score };
export type TestMembershipResult = {
	success: boolean;
	memberships: TestMembership[];
	error?: string;
};
type TestsViewProps = {
	session_id?: string;
};

export default function ViewTests(props: TestsViewProps) {
	const [tests] = createResource(
		() => props.session_id,
		async (id) => {
			if (!id) return { success: false, memberships: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/tests/get_assigned?session_id=${id}`
			);
			const resJson = (await res.json()) as TestMembershipResult;
			console.log(resJson);
			return resJson;
		}
	);
	const mappedTests = createMemo(() =>
		tests()?.memberships.map((m) => {
			return { ...m.test, score: m.score?.score };
		})
	);
	const navigate = useNavigate();
	return (
		// <div class="flex flex-col rounded-xl">
		// 	<h2>My Tests</h2>
		// 	<For each={tests()?.memberships}>
		// 		{(item, i) => {
		// 			return <div>{item.test.name}</div>;
		// 		}}
		// 	</For>
		// </div>
		<TestsView
			tests={mappedTests() ?? []}
			onTestClicked={(test) => {
				const id = test.id.split(":")[1];
				if (!id) return;
				navigate(`/user/test/${id}`);
			}}
		/>
	);
}
