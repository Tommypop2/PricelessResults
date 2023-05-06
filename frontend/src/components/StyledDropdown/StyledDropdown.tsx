import { DropdownMenu } from "@kobalte/core";
import { ParentComponent } from "solid-js";
export const DropdownItem: ParentComponent = (props) => {
	return <DropdownMenu.Item>{props.children}</DropdownMenu.Item>;
};
