import { For, createResource } from "solid-js";
import { User } from "~/context/UserProvider";
export type Test = {
	name: string;
	max_score: number;
	id: string;
	assignees: number;
};
export type TestMembership = { test: Test, user?: User };
export type TestMembershipResult = {
	success: boolean;
	memberships: TestMembership[];
	error?: string;
};
type TestsViewProps = {
	session_id?: string;
};
export default function TestsView(props: TestsViewProps) {
	const [tests] = createResource(
		() => props.session_id,
		async (id) => {
			if (!id) return { success: false, memberships: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/tests/get_assigned?session_id=${id}`
			);
			const resJson = (await res.json()) as TestMembershipResult;
			return resJson;
		}
	);
	return (
		<div class="flex flex-col rounded-xl">
			<h2>My Tests</h2>
			<For each={tests()?.memberships}>
				{(item, i) => {
					return <div>{item.test.name}</div>;
				}}
			</For>
		</div>
	);
}
