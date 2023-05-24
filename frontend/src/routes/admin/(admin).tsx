import { useNavigate } from "solid-start";

export default function Admin() {
    const navigator = useNavigate();
	return (
		<div>
			<button onClick={() => {navigator("create_test")}}>Create a test</button>
		</div>
	);
}
