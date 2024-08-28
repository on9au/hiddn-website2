import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthStatus } from '../auth';
import { LoginPayload } from '../bindings';
import FrontpageHeader from './frontpageheader';
import CommonLink from './commonlink';

import ICON from '../assets/hiddn_icon.svg';

const apiURL: string = import.meta.env.VITE_API_URL;

const Login: React.FC = () => {
    // const ICON = '../assets/hiddn_icon.svg'; // Update with your correct path to the icon

    const navigate = useNavigate();

    const [email, setEmail] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [authStatus, setAuthStatus] = useState<AuthStatus>({ type: 'Idle' });

    const handleLogin = async () => {
        setAuthStatus({ type: 'Loading' });

        const payload: LoginPayload = {
            email,
            password,
        };

        if (payload.email === '' || !payload.email.includes('@')) {
            setAuthStatus({ type: 'Error', message: 'Please enter a valid email address.' });
            return;
        }

        // Log into user, with API endpoint.
        try {
            const result = await fetch(`${apiURL}/login_user`);
            console.log(result);
            switch (result.status) {
                case 200:
                    setAuthStatus({ type: 'Success' });
                    navigate('/dashboard');
                    break;
                case 401:
                    setAuthStatus({ type: 'Error', message: 'Please check your credentials and try again.' });
                    break;
                default:
                    setAuthStatus({ type: 'Error', message: 'An error occurred.' });
            }
        } catch (error) {
            setAuthStatus({ type: 'Error', message: 'An error occurred. ' + error });
        }
    };

    const handleKeyDownLogin = async (e: React.KeyboardEvent) => {
        if (password === '') {
            return;
        }
        if (e.key === 'Enter') {
            await handleLogin();
        }
    };

    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100">
            <FrontpageHeader icon={ICON} title="HiddN" />

            <h1 className="mb-4 text-4xl">Login</h1>

            <div className="flex flex-col items-center justify-center w-full max-w-96">
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="email"
                    placeholder="Email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <input
                    className="w-full px-4 py-2 mb-4 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="password"
                    placeholder="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    onKeyDown={handleKeyDownLogin}
                />
                <button
                    className="w-full px-4 py-2 mb-4 text-white bg-hiddn-500 hover:bg-hiddn-400 rounded-xl"
                    onClick={handleLogin}
                >
                    {authStatus.type === 'Loading' ? 'Logging in...' : 'Login'}
                </button>
                {authStatus.type === 'Error' && (
                    <div className="mb-4 text-red-500">
                        {authStatus.type === 'Error' && authStatus.message}
                    </div>
                )}
                {authStatus.type === 'Success' && (
                    <div className="mb-4 text-green-500">
                        Success!
                    </div>
                )}
                <div className="flex flex-row space-x-4">
                    <CommonLink to="/register" text="Register" />
                    <CommonLink to="/forgot" text="Forgot password" />
                </div>
            </div>
        </div>
    );
};

export default Login;
