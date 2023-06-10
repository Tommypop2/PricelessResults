export type Class = { name: string };
type ClasesViewProps = {
	classes?: Class[];
};
export default function ClassesView(props: ClasesViewProps) {
	return <div>Classes: {JSON.stringify(props.classes)}</div>;
}
