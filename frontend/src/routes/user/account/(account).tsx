import { Show } from "solid-js";
import { useUserContext } from "~/context/UserProvider";
import SessionsView from "./SessionsView";
export default function Account() {
	const userCtx = useUserContext();
	return (
		<>
			<div>You're currently logged in as {userCtx.user()?.username}</div>
			<Show when={userCtx.user()?.session_id}>
				<h1>Sessions</h1>
				<div class="grid grid-flow-col grid-cols-3">
					<SessionsView
						session_id={userCtx.user()?.session_id}
						deleteSession={(session_id: string) => {
							// Also logs out user if they delete their current session
							if (userCtx.user()?.session_id === session_id) {
								userCtx.logout();
								return;
							}
							userCtx.invalidateSession(session_id);
						}}
					/>
				</div>
			</Show>
		</>
	);
}
