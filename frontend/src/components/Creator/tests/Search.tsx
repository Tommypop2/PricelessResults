import { createSignal } from "solid-js";

interface SearchProps {
	onChange: (str: string) => void;
}
export default function Search(props: SearchProps) {
	const [inp, setInp] = createSignal<HTMLInputElement>();
	return (
		<form
			class="w-full flex flex-row gap-1"
			onSubmit={(e) => {
				e.preventDefault();
			}}
		>
			<input
				class="dark:text-light-50 focus:shadow-outline rounded border leading-tight text-gray-700 shadow focus:outline-none dark:bg-[#1e1e1e] w-full"
				placeholder="Search"
				ref={setInp}
				onInput={(e) => {
					e.preventDefault();
					const str = inp()?.value;
					props.onChange(str ?? "");
				}}
			/>
			<button type="submit">Search</button>
		</form>
	);
}
