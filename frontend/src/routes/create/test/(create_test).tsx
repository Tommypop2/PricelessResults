import { createSignal } from "solid-js";
import { useUserContext } from "~/context/UserProvider";
import { createTest } from "~/helpers/tests/tests";

export default function CreateTest() {
	const userCtx = useUserContext();
	const [name, setName] = createSignal("");
	const [maxScore, setMaxScore] = createSignal<number>();
	return (
		<>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					const max = maxScore();
					if (max === undefined) return;
					createTest(
						{ name: name(), id: "jkhkjhLHKDSG1", max_score: max },
						userCtx.user()?.session_id
					);
				}}
			>
				<input
					type="text"
					placeholder="Name"
					oninput={(inp) => {
						setName(inp.target.value);
					}}
				></input>
				<input
					type="text"
					placeholder="Max Score"
					oninput={(inp) => {
						const result = parseInt(inp.target.value);
						if (Number.isNaN(result)) {
							setMaxScore(undefined);
							return;
						}
						setMaxScore(result);
					}}
				></input>
				<button type="submit">Create</button>
			</form>
		</>
	);
}
