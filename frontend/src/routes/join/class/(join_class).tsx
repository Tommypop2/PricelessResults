import { createEffect, createMemo, createResource } from "solid-js";
import { useParams, useSearchParams } from "solid-start";
import { useUserContext } from "~/context/UserProvider";

export default function JoinClass() {
	const [params] = useSearchParams();
	const userCtx = useUserContext();
	const [joinClass] = createResource(userCtx.user, async (usr) => {
		const res = await (
			await fetch(`${import.meta.env.VITE_SERVER_URI}/class/join`, {
				method: "post",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					session_id: usr.session_id,
					class_id: params.class_id,
				}),
			})
		).json();
        console.log(res);
		return res;
	});
	const yes = createMemo(() => joinClass());
	return <></>;
}
