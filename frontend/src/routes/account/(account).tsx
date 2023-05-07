import { Show, Suspense } from "solid-js";
import { useUserContext } from "~/context/UserProvider";
function Yes(props: { user: any }) {
	return <div>{props.user.userName}</div>;
}
export default function Account() {
	const userCtx = useUserContext();
	return (
		<Suspense>
			<div>{userCtx.user()?.username}</div>
		</Suspense>
	);
}
