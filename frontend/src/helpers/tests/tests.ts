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
// CRUD Operations
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
	const id = test_id.split(":")[1];
	if (!session_id || !id) return null;
	const res = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/tests/delete`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ test_id: id, session_id }),
		})
	).json();
	return res;
}

// Assignments
type AssignTestParams = {
	session_id: string;
	test_id: string;
	class_id?: string;
	user_id?: string;
};
export async function assignTest(
	test_id: string,
	class_id: string,
	session_id?: string
) {
	if (!session_id) return null;
	const res = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/tests/assign`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({
				test_id,
				class_id,
				session_id,
			} as AssignTestParams),
		})
	).json();
	return res;
}
