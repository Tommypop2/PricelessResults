type Test = {
	creation_date: Date;
	name: string;
	creator: string;
	max_score: number;
	id: string;
};
type CreateTestResult = {
	success: boolean;
	test: Test | null;
	error: string | null;
};
export async function createTest(
	test: Omit<Test, "id" | "creation_date" | "creator">,
	session_id?: string
) {
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
export async function deleteTest(test_id: string, session_id?: string) {
	if (!session_id) return null;
	const res = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/tests/delete`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ test_id, session_id }),
		})
	).json();
	return res;
}
