import { Show, Suspense, createResource } from "solid-js";
import { useNavigate } from "solid-start";
import ClassesView, { Class } from "~/components/classes/UserClasses";
import TestsView, { Test } from "~/components/tests/TestsView";
import { useUserContext } from "~/context/UserProvider";

export default function Admin() {
	const userCtx = useUserContext();
	const session_id = () => userCtx.user()?.session_id;
	const [tests] = createResource(session_id, async (id) => {
		const res = await fetch(
			`${import.meta.env.VITE_SERVER_URI}/tests?session_id=${id}`
		);
		const resJson = await res.json();
		return resJson["tests"] as Test[];
	});
	const [classes] = createResource(session_id, async (id) => {
		const res = await fetch(
			`${import.meta.env.VITE_SERVER_URI}/class/get?session_id=${id}`
		);
		const resJson = await res.json();
		return resJson["classes"] as Class[];
	});
	return (
		<div class="grid grid-cols-3">
			<Show when={userCtx.user()?.session_id}>
				<TestsView tests={tests()} />
				{/* <ClassesView classes={classes()} /> */}
			</Show>
		</div>
	);
}
