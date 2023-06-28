import { For, createResource } from "solid-js";
import { useSearchParams } from "solid-start";
import { TestMembershipResult } from "~/components/User/tests/UserTests";
import { useUserContext } from "~/context/UserProvider";
import { createScore } from "~/helpers/scores/scores";

export default function AddScores() {
	const [params] = useSearchParams();
	const userCtx = useUserContext();
	const session_id = () => userCtx.user()?.session_id;
	const [users] = createResource(session_id, async (session_id) => {
		const res = await fetch(
			`${
				import.meta.env.VITE_SERVER_URI
			}/tests/get_assigned_in_class?session_id=${session_id}&class_id=${
				params.class_id
			}&test_id=${params.test_id}`
		);
		const resJson = await res.json();
		return resJson as TestMembershipResult;
	});
	return (
		<div>
			<table>
				<thead>
					<tr>
						<th>User</th>
						<th>Score</th>
					</tr>
				</thead>
				<tbody>
					<For each={users()?.memberships!}>
						{(test) => {
							const user = test.user;
							return (
								<tr>
									<td>{user?.username}</td>
									<td>
										<input
											onChange={async (e) => {
												const val = parseInt(e.currentTarget.value);
												const id = test.test.id.split(":")[1];
												if (!id || !val) return;
												const res = await createScore(
													{
														test_id: id,
														user_id: user?.user_id!,
														score: val,
													},
													session_id()
												);
											}}
										/>
									</td>
								</tr>
							);
						}}
					</For>
				</tbody>
			</table>
		</div>
	);
}
