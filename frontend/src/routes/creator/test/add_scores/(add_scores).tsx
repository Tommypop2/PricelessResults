import { For, createResource } from "solid-js";
import { useSearchParams } from "solid-start";
import { TestMembershipResult } from "~/components/User/tests/UserTests";
import { useUserContext } from "~/context/UserProvider";
import { createScore } from "~/helpers/scores/scores";
type ScoreRecord = {
	id: string;
	score: number;
	test: string;
	user: string;
	creation_date: string;
};
type ScoreResult = {
	success: boolean;
	error?: string;
	scores: ScoreRecord[];
};
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
	const [scores] = createResource(session_id, async (session_id) => {
		const res = await fetch(
			`${
				import.meta.env.VITE_SERVER_URI
			}/score/read?session_id=${session_id}&class_id=${
				params.class_id
			}&test_id=${params.test_id}`
		);
		const resJson = await res.json();
		return resJson as ScoreResult;
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
							const numbersOnly = /\d+/g;
							const score = scores()?.scores.find(
								(record) =>
									record.user.match(numbersOnly)?.[0] === user?.user_id
							);
							return (
								<tr>
									<td>{user?.username}</td>
									<td>
										<input
											value={score?.score ?? ""}
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
												console.log(res);
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
