import { createSignal } from "solid-js";
import { Toaster, toast } from "solid-toast";
import { useUserContext } from "~/context/UserProvider";
import { createClass } from "~/helpers/classes/class";

export default function CreateClass() {
	const userCtx = useUserContext();
	const [name, setName] = createSignal("");
	return (
		<>
			<form
				onsubmit={async (e) => {
					e.preventDefault();
					const res = await createClass(
						{ name: name() },
						userCtx.user()?.session_id
					);
					if (res?.success) {
						console.log("Toasting");
						toast.success("Class created successfully!");
					}
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
