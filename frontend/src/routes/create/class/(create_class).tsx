import { createSignal } from "solid-js";
import { useUserContext } from "~/context/UserProvider";
import { createClass } from "~/helpers/classes/class";

export default function CreateClass() {
	const userCtx = useUserContext();
	const [name, setName] = createSignal("");
	return (
		<>
			<form
				onsubmit={(e) => {
					e.preventDefault();
					createClass(
						{ name: name() },
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
				<button type="submit">Create</button>
			</form>
		</>
	);
}
