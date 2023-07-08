import {
	Suspense,
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
	Test,
	TestMembershipResult,
} from "~/components/User/tests/UserTests";
import Container from "~/components/Container/Container";
import { useThemeContext } from "~/context/ThemeProvider";
export default function Dashboard() {
	const userCtx = useUserContext();
	const themeCtx = useThemeContext();
	// User scores
	const [testsRes] = createResource(
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
	const tests = createMemo(() =>
		testsRes()?.memberships?.map((m) => {
			return { ...m.test, score: m.score?.score };
		})
	);
	// Class averages
	const [averages] = createResource(
		() => userCtx.user()?.session_id,
		async (id) => {
			const class_id = "qd9pnvtq0bdd6zppowqu";
			const res = await fetch(
				`${
					import.meta.env.VITE_SERVER_URI
				}/statistics/class_average?session_id=${id}&class_id=${class_id}`
			);
			const resJson = await res.json();
			return resJson as { test: Test; mean_score: number }[];
		}
	);
	onMount(() => {
		Chart.register(Title, Tooltip, Legend, Colors);
	});
	const pairs = () =>
		tests()?.map((t) => {
			const corresponding = averages()?.find((a) => a.test.id === t.id);
			return {
				test: t,
				mean: corresponding ? corresponding.mean_score : null,
			};
		});

	const pointBgColour = () => (themeCtx.theme() === "dark" ? "white" : "black");
	const chartData: () => ChartData = () => ({
		xLabels: pairs()?.map((p) => p.test.name),
		datasets: [
			{
				label: "Test Scores",
				data: pairs()?.map((t) =>
					t.test.score ? (t.test.score / t.test.max_score) * 100 : null
				)!,
				pointBackgroundColor: pointBgColour(),
				borderColor: "red",
				tension: 0.2,
			},
			{
				label: "Class Averages",
				data:
					pairs()?.map((t) => {
						return t.mean ? (t.mean / t.test.max_score) * 100 : null;
					}) ?? [],
				pointBackgroundColor: pointBgColour(),
				borderColor: "blue",
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
	const reversedTests = createMemo(() =>
		tests() ? [...tests()!.reverse()] : undefined
	);
	return (
		<div class="grid grid-cols-4 p-2 gap-2">
			<div class="transition-all ease-in-out col-span-2">
				<Suspense>
					<Line data={chartData()} options={chartOptions} />
				</Suspense>
			</div>
			<Container class="text-left">
				<Suspense>
					<ClassesView session_id={session_id()} />
				</Suspense>
			</Container>
			<Container class="text-left">
				<Suspense>
					<ViewTests tests={reversedTests()} />
				</Suspense>
			</Container>
			<div>Slot 4</div>
			<div>Slot 5</div>
		</div>
	);
}
