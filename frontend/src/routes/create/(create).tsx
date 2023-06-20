import { A } from "solid-start";
import ClassesView from "~/components/Creator/classes/ViewClasses";
import { useUserContext } from "~/context/UserProvider";

export default function Create() {
	const userCtx = useUserContext();
	return (
		<div class="flex flex-col">
			<ClassesView session_id={userCtx.user()?.session_id}/>
		</div>
	);
}
