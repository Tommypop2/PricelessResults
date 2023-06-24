import { For } from "solid-js";
import { Td, Th, Tr } from "./StyledTable";
interface TableProps {
	columns: Record<string, string>;
	// This is very difficult to properly type because the object could contain other object, or it just couldn't
	data: any[];
	resolveChildren?: (item: any) => any;
}
export function Table(props: TableProps) {
	function resolveChild(item: any, key: string) {
		// Handle button as a special case, because the onclick handler needs to have parameters injected
		const keys = key.split(".");
		if (keys.length === 0) return item;
		let el = item[keys[0]];
		for (const k of keys.slice(1)) {
			const temp = el[k];
			if (temp.type === "button") {
				return <button onClick={() => temp.onClick(item)}>{temp.children}</button>;
			}
			if (!temp) break;
			el = temp;
		}
		return el;
	}
	const columnKeys = () => Object.keys(props.columns);
	return (
		<table>
			<thead>
				<Tr>
					<For each={columnKeys()}>{(item) => <Th>{item}</Th>}</For>
				</Tr>
			</thead>
			<tbody>
				<For each={props.data}>
					{(item) => (
						<tr>
							<For each={columnKeys()}>
								{(key) => {
									return <Td>{resolveChild(item, props.columns[key])}</Td>;
								}}
							</For>
						</tr>
					)}
				</For>
			</tbody>
		</table>
	);
}
