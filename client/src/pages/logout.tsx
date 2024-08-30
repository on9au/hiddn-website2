import React, { useEffect } from "react";
import { LogoutStatus } from "../auth";
import { useNavigate } from "react-router-dom";

const Logout: React.FC = () => {
    const [logoutStatus, setLogoutStatus] = React.useState<LogoutStatus>({ type: 'Idle' });
    const navigate = useNavigate();

    useEffect(() => {
        // Log out user, with API endpoint.
        setLogoutStatus({ type: 'LoggingOut' });
        fetch(`api/logout_user`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        }).then((response) => {
            if (response.status === 200) {
                setLogoutStatus({ type: 'Success' });
                localStorage.removeItem('isAuthenticated');
                navigate('/login')
            } else {
                setLogoutStatus({ type: 'Error', message: response.statusText });
            }
        });
    }, [navigate]);


    return (
        <>
            <h1 className="mb-4 text-4xl">Logging you out...</h1>
            {logoutStatus.type === 'Error' && <p className="text-red-500">{logoutStatus.message}</p>}
            {logoutStatus.type === 'Success' && <p className="text-green-500">Logged out successfully. Redirecting...</p>}
        </>
    );
};

export default Logout;