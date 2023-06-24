import { createSignal } from "solid-js";
import { FiPlus } from "solid-icons/fi";
interface AddTestProps {
	onAddTest: (testName?: string, maxScore?: number) => Promise<boolean>;
}
export default function AddTest(props: AddTestProps) {
	const [testNameInp, setTestNameInp] = createSignal<HTMLInputElement>();
	const [maxScoreInp, setMaxScoreInp] = createSignal<HTMLInputElement>();
	return (
		<form
			onSubmit={async (e) => {
				e.preventDefault();
				const nameInp = testNameInp();
				const scoreInp = maxScoreInp();
				if (!nameInp || !scoreInp) return;
				const testName = nameInp.value;
				const maxScore = parseInt(scoreInp.value);
				if (await props.onAddTest(testName, maxScore)) {
					nameInp.value = "";
                    scoreInp.value = "";
				}
			}}
			class="w-full flex flex-row gap-1"
		>
			<input
				ref={setTestNameInp}
				placeholder="Test Name"
				class="dark:text-light-50 focus:shadow-outline rounded border leading-tight text-gray-700 shadow focus:outline-none dark:bg-[#1e1e1e] w-full"
			/>
			<input
				ref={setMaxScoreInp}
				placeholder="Max Score"
				class="dark:text-light-50 focus:shadow-outline rounded border leading-tight text-gray-700 shadow focus:outline-none dark:bg-[#1e1e1e] w-full"
			/>
			<button
				type="submit"
				class="bg-transparent text-inherit border-none active:animate-rubber-band"
			>
				<FiPlus size={25} />
			</button>
		</form>
	);
}
