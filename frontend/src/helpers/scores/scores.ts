import { Result } from "../types";

export type Score = {
	user: string;
	test: string;
	score: number;
};
export interface ScoreResult extends Result {
	score: Score | null;
}
export interface ScoresResult extends Result {
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
export async function getUserScores(session_id?: string) {
	if (!session_id) return null;
	const res = await (
		await fetch(
			`${
				import.meta.env.VITE_SERVER_URI
			}/score/read_user?session_id=${session_id}`
		)
	).json();
	return res as ScoresResult;
}
