import { Result } from "../types";

type Class = {
	name: string;
	creation_date: Date;
	creator: string;
	id: string;
};
interface CreateClassResult extends Result {
	class: Class | null
}
export async function createClass(
	user_class: Omit<Class, "id" | "creation_date" | "creator">,
	session_id?: string
) {
	if (!session_id) return null;
	const res: CreateClassResult = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/class/create`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ class: user_class, session_id }),
		})
	).json();
	return res;
}
type DeleteClassResult = {
	success: boolean;
	error?: string;
};
export async function deleteClass(class_id: string, session_id?: string) {
	if (!session_id) return null;
	const res: DeleteClassResult = await (
		await fetch(`${import.meta.env.VITE_SERVER_URI}/class/delete`, {
			credentials: "include",
			mode: "cors",
			method: "post",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ class_id, session_id }),
		})
	).json();
	return res;
}
