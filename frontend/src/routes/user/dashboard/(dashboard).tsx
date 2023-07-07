import {
	createEffect,
	createMemo,
	createResource,
	onCleanup,
	onMount,
} from "solid-js";
import {
	Chart,
	Title,
	Tooltip,
	Legend,
	Colors,
	ChartData,
	ChartOptions,
} from "chart.js";
import { Line } from "solid-chartjs";
import ClassesView from "~/components/User/classes/UserClasses";
import { useUserContext } from "~/context/UserProvider";
import ViewTests, {
	TestMembershipResult,
} from "~/components/User/tests/UserTests";
import Container from "~/components/Container/Container";
export default function Dashboard() {
	const userCtx = useUserContext();
	const [tests] = createResource(
		() => userCtx.user()?.session_id,
		async (id) => {
			if (!id) return { success: false, memberships: [] };
			const res = await fetch(
				`${import.meta.env.VITE_SERVER_URI}/tests/get_assigned?session_id=${id}`
			);
			const resJson = (await res.json()) as TestMembershipResult;
			return resJson;
		}
	);
	const mappedTests = createMemo(() =>
		tests()?.memberships?.map((m) => {
			return { ...m.test, score: m.score?.score };
		})
	);
	onMount(() => {
		Chart.register(Title, Tooltip, Legend, Colors);
	});
	const chartData: () => ChartData = () => ({
		xLabels: mappedTests()?.map((t) => t.name),
		datasets: [
			{
				label: "Test Scores",
				data: mappedTests()?.map((t) =>
					t.score ? (t.score / t.max_score) * 100 : null
				)!,
				pointBackgroundColor: "yellow",
				borderColor: "red",
				tension: 0.2,
			},
			{
				label: "Class Averages",
				data: [69, 90, 70, 65, 84],
				pointBackgroundColor: "yellow",
				borderColor: "green",
				tension: 0.2,
			},
		],
	});
	const chartOptions: ChartOptions = {
		scales: {
			y: {
				display: true,
				title: {
					display: true,
					text: "Score (%)",
				},
				max: 100,
				min: 0,
			},
			x: {},
		},
		responsive: true,
		maintainAspectRatio: false,
	};
	const session_id = () => userCtx.user()?.session_id;
	return (
		<div class="grid grid-cols-4 p-2 gap-2">
			<div class="transition-all ease-in-out col-span-2">
				<Line data={chartData()} options={chartOptions} />
			</div>
			<Container>
				<ClassesView session_id={session_id()} />
			</Container>
			<Container>
				<ViewTests tests={mappedTests()} />
			</Container>
			<div>Slot 4</div>
			<div>Slot 5</div>
		</div>
	);
}
