import { IconTypes } from "solid-icons";
import { ImBin } from "solid-icons/im";
import {
	For,
	JSX,
	Show,
	createMemo,
	createResource,
	createSignal,
} from "solid-js";
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
	tests: Test[];
	// updateTests: (newTests: Test[]) => void;
	onTestClicked: (test: Test) => void;
	onButtonClicked?: (test: Test) => void;
	buttonIcon?: IconTypes;
	buttonTitle?: string;
}
export function TestsView(props: ViewTestsProps) {
	const tests = () => props.tests;
	const includesScores = createMemo(() => {
		for (const test of tests()) {
			if (test.score) return true;
		}
		return false;
	});
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
					<Show when={includesScores()}>
						<th>
							<h2>Score</h2>
						</th>
					</Show>
					<Show when={props.buttonIcon}>
						<th class="w-[44px]"></th>
					</Show>
				</tr>
			</thead>
			<tbody>
				<For each={tests()}>
					{(item) => (
						<tr
							onClick={() => {
								props.onTestClicked(item);
							}}
						>
							<td class="text-xl">{item.name}</td>
							<td class="text-xl">{item.max_score}</td>
							<td class="text-xl">{item.assignees}</td>
							<Show when={includesScores()}>
								<td class="text-xl">{item.score}</td>
							</Show>
							<Show when={props.buttonIcon}>
								<td>
									<button
										class="rounded bg-transparent border-none active:animate-jello animate-duration-75"
										onClick={async (e) => {
											e.stopPropagation();
											props.onButtonClicked?.(item);
										}}
										title={props.buttonTitle}
									>
										{(() => {
											if (!props.buttonIcon) return;
											return <props.buttonIcon size={30} />;
										})()}
									</button>
								</td>
							</Show>
						</tr>
					)}
				</For>
			</tbody>
		</table>
	);
}
