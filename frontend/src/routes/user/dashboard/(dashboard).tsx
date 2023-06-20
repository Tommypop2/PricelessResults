import { onMount } from "solid-js";
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
import TestsView from "~/components/User/tests/UserTests";
import Container from "~/components/Container/Container";
export default function Dashboard() {
	const userCtx = useUserContext();
	onMount(() => {
		Chart.register(Title, Tooltip, Legend, Colors);
	});

	const chartData: ChartData = {
		labels: ["Test1", "Test2", "Test3", "Test4", "Test5"],
		datasets: [
			{
				label: "Test Scores",
				data: [80, 75, 90, 50, 72],
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
	};
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
				<Line data={chartData} options={chartOptions} />
			</div>
			<Container>
				<ClassesView session_id={session_id()} />
			</Container>
			<Container>
				<TestsView session_id={session_id()} />
			</Container>
			<div>Slot 4</div>
			<div>Slot 5</div>
		</div>
	);
}
