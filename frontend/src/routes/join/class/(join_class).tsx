import { createEffect, createMemo, createResource } from "solid-js";
import { useNavigate, useSearchParams } from "solid-start";
import toast from "solid-toast";
import { useUserContext } from "~/context/UserProvider";

export default function JoinClass() {
	const [params] = useSearchParams();
	const userCtx = useUserContext();
	const navigate = useNavigate();
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
		return res;
	});
	createMemo(() => joinClass());
	createEffect(() => {
		joinClass();
		if (joinClass.loading) return;
		if (!joinClass()?.success) {
			toast.error(joinClass()?.error);
			navigate("/user/dashboard");
		} else {
			toast.success("Class joined successfully");
			navigate("/user/dashboard");
		}
	});
	return <></>;
}
