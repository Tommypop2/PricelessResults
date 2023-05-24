import { useUserContext } from "~/context/UserProvider";
import { createTest } from "~/helpers/tests/tests";

export default function CreateTest() {
	const userCtx = useUserContext();
	return (
		<button
			onClick={() => {
				createTest(
					{ name: "test1", id: "jkhkjhLHKDSG1", max_score: 50 },
					userCtx.user()?.session_id
				);
			}}
		>
			Create test Test
		</button>
	);
}
