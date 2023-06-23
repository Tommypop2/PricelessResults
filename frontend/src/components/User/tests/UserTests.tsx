import { For, createResource } from "solid-js";
export type Test = { name: string; max_score: number; id: string };
type TestMembership = { test: Test };
type TestMembershipResult = {
	success: boolean;
	memberships: TestMembership[];
	error?: string;
};
type ClasesViewProps = {
	session_id?: string;
};
export default function TestsView(props: ClasesViewProps) {
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
