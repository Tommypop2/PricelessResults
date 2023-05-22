import { SolidApexCharts } from "solid-apexcharts";
import { createStore } from "solid-js/store";
// This file must be kept separate from the routes directoy, as the file system router likes to mess with it
export default function Charts() {
	const [options] = createStore({
		chart: {
			id: "solidchart-example",
		},
		xaxis: {
			categories: [1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998],
		},
	});
	const [series] = createStore({
		list: [
			{
				name: "series-1",
				data: [30, 40, 35, 50, 49, 60, 70, 91],
			},
		],
	});

	// options and series can be a store or signal
	return (
		<>
			Cool Dashboard Page
			<SolidApexCharts
				width="500"
				type="bar"
				options={options}
				series={series.list}
			/>
		</>
	);
}
