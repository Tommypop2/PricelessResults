import { ImBin } from "solid-icons/im";
import { For, createResource, createSignal } from "solid-js";
import toast from "solid-toast";
import { Test } from "~/components/User/tests/UserTests";
import { createTest } from "~/helpers/tests/tests";
type GetCreatedResult = {
	success: boolean;
	error?: string;
	tests: Test[];
};
interface ViewTestsProps {
	session_id?: string;
}
export function ViewTests(props: ViewTestsProps) {
	const [tests] = createResource(
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
	const [nameRef, setNameRef] = createSignal<HTMLInputElement>();
	const [maxScoreRef, setMaxScoreRef] = createSignal<HTMLInputElement>();
	return (
		<div class="flex flex-col rounded-xl">
			<h2>My Tests</h2>
			<For each={tests()?.tests}>
				{(item, i) => {
					// This sucks, but it's ok for prototyping
					return (
						<div>
							{item.name}
							{"  "}
							{item.max_score}
						</div>
					);
				}}
			</For>
			<form
				onSubmit={async (e) => {
					e.preventDefault();
					const nameEl = nameRef();
					const maxScoreEl = maxScoreRef();
					if (!nameEl || !maxScoreEl) return;
					const name = nameEl.value;
					const max_score = parseInt(maxScoreEl.value);
					if (!name || !max_score) return;
					await createTest({ name, max_score }, props.session_id!);
				}}
			>
				<input placeholder="Name" ref={setNameRef}></input>
				<input placeholder="Max Score" ref={setMaxScoreRef}></input>
				<input type="submit"></input>
			</form>
		</div>
	);
}
