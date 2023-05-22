import {
	For,
	Show,
	Suspense,
	createEffect,
	createResource,
	on,
} from "solid-js";
type SessionsViewProps = {
	session_id?: string;
	deleteSession: (session_id: string) => void;
};
type Session = { session_id: string; user_id: string; user_agent: string };
export default function SessionsView(props: SessionsViewProps) {
	const [user_sessions, { mutate }] = createResource(async () => {
		if (!props.session_id) return;
		const res = await fetch(
			`${import.meta.env.VITE_SERVER_URI}/user/sessions?session_id=${
				props.session_id
			}`
		);
		const resJson = (await res.json())["sessions"] as Session[];
		return resJson;
	});

	return (
		<Suspense>
			<For each={user_sessions()}>
				{(item) => {
					return (
						<div class="flex flex-col">
							<Show when={item.session_id === props.session_id}>
								This is your current session
							</Show>
							<span>{item.session_id}</span>
							<span>{item.user_agent}</span>
							<button
								onClick={() => {
									let sessions = user_sessions();
									props.deleteSession(item.session_id);
									sessions = sessions?.filter(
										(session) => session.session_id != item.session_id
									);
									mutate(sessions);
								}}
							>
								Delete Session
							</button>
						</div>
					);
				}}
			</For>
		</Suspense>
	);
}
