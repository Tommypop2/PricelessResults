type Test = {
	name: string;
	max_score: number;
	id: string;
};
type CreateTestResult = {
	success: boolean;
	test: Test | null;
	error: string | null;
};
export async function createTest(test: Test, session_id?: string) {
	if (!session_id) return null;
	const res: CreateTestResult = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/tests/create`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ test, session_id }),
		})
	).json();
	return res;
}