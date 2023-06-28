import { IconTypes } from "solid-icons";
import { ImBin } from "solid-icons/im";
import { For, JSX, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import AddTest from "~/components/AddTest/AddTest";
import { Test } from "~/components/User/tests/UserTests";
import { createTest, deleteTest } from "~/helpers/tests/tests";
export type GetCreatedResult = {
	success: boolean;
	error?: string;
	tests: Test[];
};
interface ViewTestsProps {
	session_id?: string;
	tests: GetCreatedResult;
	updateTests: (newTests: Test[]) => void;
	onTestClicked: (test: Test) => void;
	onButtonClicked: (test: Test) => void;
	buttonIcon: IconTypes;
	buttonTitle: string;
}
export function TestsView(props: ViewTestsProps) {
	const tests = () => props.tests;
	return (
		<table class="w-full">
			<thead>
				<tr>
					<th class="w-[1fr]">
						<h2>My Tests</h2>
					</th>
					<th class="w-[1fr]">
						<h2>Max Score</h2>
					</th>
					<th>
						<h2>Assignees</h2>
					</th>
					<th class="w-[44px]"></th>
				</tr>
			</thead>
			<tbody>
				<For each={tests()?.tests}>
					{(item) => (
						<tr
							onClick={() => {
								props.onTestClicked(item);
							}}
						>
							<td class="text-xl">{item.name}</td>
							<td class="text-xl">{item.max_score}</td>
							<td class="text-xl">{item.assignees}</td>
							<td>
								<button
									class="rounded bg-transparent border-none active:animate-jello animate-duration-75"
									onClick={async (e) => {
										e.stopPropagation();
										props.onButtonClicked(item);
									}}
									title={props.buttonTitle}
								>
									<props.buttonIcon size={30} />
								</button>
							</td>
						</tr>
					)}
				</For>
			</tbody>
		</table>
	);
}
