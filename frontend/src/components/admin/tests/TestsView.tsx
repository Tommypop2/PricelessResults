import { For } from "solid-js";

export type Test = {
	id: string;
	name: string;
	max_score: string;
	creator: string;
};
type TestsViewProps = {
	tests?: Test[];
};
export default function TestsView(props: TestsViewProps) {
	return (
		<div class="flex-col">
			<h1 class="px-5">Tests</h1>
			<table>
				<thead>
					<tr>
						<th>Name</th>
						<th>Max Score</th>
					</tr>
				</thead>
				<tbody>
					<For each={props.tests}>
						{(test, index) => {
							return (
								<tr>
									<td>{test.name}</td>
                                    <td>{test.max_score}</td>
								</tr>
							);
						}}
					</For>
				</tbody>
			</table>
		</div>
	);
}
