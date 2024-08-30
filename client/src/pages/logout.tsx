import React, { useEffect } from "react";
import FrontpageHeader from "../components/frontpageheader";

import ICON from '../assets/hiddn_icon.svg';
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
                navigate('/login')
            } else {
                setLogoutStatus({ type: 'Error', message: response.statusText });
            }
        });
    }, []);


    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100">
            <FrontpageHeader icon={ICON} title="HiddN" />

            <h1 className="mb-4 text-4xl">Logging you out...</h1>
            {logoutStatus.type === 'Error' && <p className="text-red-500">{logoutStatus.message}</p>}
            {logoutStatus.type === 'Success' && <p className="text-green-500">Logged out successfully. Redirecting...</p>}
        </div>
    );
};

export default Logout;