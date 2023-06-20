import { Chart, ChartData, ChartOptions, Colors, Legend, Title, Tooltip } from "chart.js";
import { Line } from "solid-chartjs";
import { onMount } from "solid-js";
import { A } from "solid-start";
import Container from "~/components/Container/Container";
import ClassesView from "~/components/Creator/classes/ViewClasses";
import { useUserContext } from "~/context/UserProvider";

export default function Create() {
	const userCtx = useUserContext();
	const session_id = () => userCtx.user()?.session_id;
	onMount(() => {
		Chart.register(Title, Tooltip, Legend, Colors);
	});

	const chartData: ChartData = {
		labels: ["Test1", "Test2", "Test3", "Test4", "Test5"],
		datasets: [
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
	return (
		<div class="grid grid-cols-4 p-2 gap-2">
			<div class="transition-all ease-in-out col-span-2">
				<Line data={chartData} options={chartOptions} />
			</div>
			<Container>
				<ClassesView session_id={session_id()} />
			</Container>
			<div>Slot 4</div>
			<div>Slot 5</div>
		</div>
	);
}
