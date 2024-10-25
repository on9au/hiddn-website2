import React, { useEffect } from "react";
import { LogoutStatus } from "../auth";
import { useNavigate } from "react-router-dom";
import Loginbutton from "../components/loginbutton";

const Goodbye: React.FC = () => {
    const navigate = useNavigate();

    useEffect(() => { document.title = 'Goodbye - HiddN'; });

    const [logoutStatus, setLogoutStatus] = React.useState<LogoutStatus>({ type: 'Idle' });

    useEffect(() => {
        // Log out user, with API endpoint.
        setTimeout(() => {
            setLogoutStatus({ type: 'LoggingOut' });
            try {
                fetch(`api/logout_user`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                }).then((response) => {
                    if (response.status === 200) {
                        setLogoutStatus({ type: 'Success' });
                        localStorage.removeItem('isAuthenticated');
                    } else {
                        setLogoutStatus({ type: 'Error', message: response.statusText });
                    }
                });
            } catch (error) {
                setLogoutStatus({ type: 'Error', message: 'Failed to log out. Try again later. Error: ' + error });
            }
        }, 5000);
    }, []);

    const returnToLogin = () => {
        navigate('/login');
    }

    return (
        <div className="flex flex-col items-center justify-center w-full max-w-96">
            <div className="flex flex-col items-center justify-center w-full text-center max-w-96">
                <h1 className="items-center mb-4 text-4xl font-semibold text-center">Account Deleted</h1>
                <p className="font-semibold text-center text-gray-700 dark:text-gray-300">
                    Your account has been successfully deleted. We're sorry to see you go.
                </p>
                <p className="mt-2 font-semibold text-center text-gray-700 dark:text-gray-300">
                    If you have any questions or feedback, feel free to <a href="/contact" className="text-blue-500 underline">contact us</a>.
                </p>
            </div>
            {logoutStatus.type !== 'Success' && logoutStatus.type !== 'Error' && (
                <>
                    <h1 className="mb-4 text-4xl">Logging you out...</h1>
                    <div
                        className="inline-block h-8 w-8 mb-4 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
                        role="status">
                        <span
                            className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                        >Loading...</span>
                    </div>
                </>
            )}
            {logoutStatus.type === 'Error' && (
                <>
                    <p className="mb-4 text-red-500">Logout failed. Don't worry, your account is still deleted. Error: {logoutStatus.message}</p>
                    <Loginbutton
                        content="To login page"
                        handleLogin={returnToLogin}
                    />
                </>
            )}
            {logoutStatus.type === 'Success' && (
                <>
                    <p className="text-green-500">Logged out and account deleted successfully.</p>
                    <Loginbutton 
                        content="To login page"
                        handleLogin={returnToLogin}
                    />
                </>
            )}
        </div>
    );
};

export default Goodbye;
