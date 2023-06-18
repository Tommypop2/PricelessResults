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
import ClassesView from "~/components/classes/UserClasses";
import { useUserContext } from "~/context/UserProvider";
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

	return (
		<>
			<ClassesView session_id={userCtx.user()?.session_id} />
			<div class="w-80 h-80 hover:scale-105 transition-all ease-in-out">
				<Line data={chartData} options={chartOptions} />
			</div>
		</>
	);
}
