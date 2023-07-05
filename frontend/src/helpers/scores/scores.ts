import { Result } from "../types";

export type Score = {
	user_id: string;
	test_id: string;
	score: number;
};
interface ScoreResult extends Result {
	score: Score | null;
}
interface ScoresResult extends Result {
	scores: Score[] | null;
}
export async function createScore(score: Score, session_id?: string) {
	if (!session_id) return null;
	const data = { ...score, session_id };
	console.log(data);
	const res = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/score/create`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify(data),
		})
	).json();
	return res as ScoreResult;
}
export async function getUserScores(user_id: string, session_id?: string) {
	if (!session_id) return null;
	const res = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/score/read`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ user_id, session_id }),
		})
	).json();
	return res as ScoresResult;
}
