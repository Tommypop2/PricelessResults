import { Navigate, Outlet } from "solid-start";
import { useUserContext } from "~/context/UserProvider";

export default function Protected() {
	const userCtx = useUserContext();
	return (
		<>
			<Outlet />
			{() => {
				if (userCtx.user.loading === false && !userCtx.user()?.admin)
					return <Navigate href="/" />;
			}}
		</>
	);
}
