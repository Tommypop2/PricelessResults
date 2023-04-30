import { A, Title } from "solid-start";
import { HttpStatusCode } from "solid-start/server";

export default function NotFound() {
	return (
		<div>
			<Title>Not Found</Title>
			<HttpStatusCode code={404} />
			<h1 class="p-0 m-0">Page Not Found</h1>
			<p>The page you're looking for doesn't exist</p>
			<A href="/">Home</A>
		</div>
	);
}
