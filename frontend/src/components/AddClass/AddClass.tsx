import { createSignal } from "solid-js";
import { FiPlus } from "solid-icons/fi";
interface AddClassProps {
	onAddClass: (className?: string) => Promise<boolean>;
}
export default function AddClass(props: AddClassProps) {
	const [classNameInp, setClassNameInp] = createSignal<HTMLInputElement>();
	return (
		<form
			onSubmit={async (e) => {
				e.preventDefault();
				const inp = classNameInp();
				if (!inp) return;
				const className = inp.value;
				if (await props.onAddClass(className)) {
					inp.value = "";
				}
			}}
			class="w-full flex flex-row"
		>
			<input
				ref={setClassNameInp}
				placeholder="Class Name"
				class="dark:text-light-50 focus:shadow-outline rounded border leading-tight text-gray-700 shadow focus:outline-none dark:bg-[#1e1e1e] w-full"
			></input>
			<button
				type="submit"
				class="bg-transparent text-inherit border-none active:animate-rubber-band"
			>
				<FiPlus size={25} />
			</button>
		</form>
	);
}
