import { For, JSX, createMemo } from "solid-js";

interface TableProps {
	columns: string[];
	data: Record<string, string>[];
}
export default function Table(props: TableProps) {
	return (
		<table>
			<thead>
				<tr>
					<For each={props.columns}>
						{(item) => (
							<th>
								<h2>{item}</h2>
							</th>
						)}
					</For>
				</tr>
			</thead>
			<tbody>
				<For each={props.data}>
					{(item) => (
						<tr>
							<For each={props.columns}>
								{(colName) => <td>{item[colName]}</td>}
							</For>
						</tr>
					)}
				</For>
			</tbody>
		</table>
	);
}
