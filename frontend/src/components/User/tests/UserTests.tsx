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
	tests?: Test[];
};

export default function ViewTests(props: TestsViewProps) {
	const navigate = useNavigate();
	return (
		<TestsView
			tests={props.tests ?? []}
			onTestClicked={(test) => {
				const id = test.id.split(":")[1];
				if (!id) return;
				navigate(`/user/test/${id}`);
			}}
		/>
	);
}
