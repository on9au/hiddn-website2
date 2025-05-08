import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthStatus } from '../auth';
import CommonLink from '../components/commonlink';
import Loginbutton from '../components/loginbutton';
import TextInput from '../components/logintextinput';
import { LoginPayload } from '../bindings/LoginPayload';

// const apiURL: string = import.meta.env.VITE_API_URL;

const Login: React.FC = () => {
    // const ICON = '../assets/hiddn_icon.svg'; // Update with your correct path to the icon

    useEffect(() => { document.title = 'Login - HiddN'; });

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

        if (payload.email === '') {
            setAuthStatus({ type: 'Error', message: 'Email cannot be blank.' });
            return;
        }

        // Abort if connection is not secure. Bypass if running on development.
        // if (!import.meta.env.DEV && !window.location.protocol.includes('https')) {
        //     setAuthStatus({ type: 'Error', message: 'Connection is not secure. Please use HTTPS.' });
        //     return;
        // }


        // Log into user, with API endpoint.
        try {
            const result = await fetch(`api/auth/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(payload),
            }).then((response) => {
                return response;
            });
            switch (result.status) {
                case 200:
                    setAuthStatus({ type: 'Success' });
                    navigate('/user/dashboard');
                    break;
                case 401:
                    setAuthStatus({ type: 'Error', message: 'Incorrect email/password.' });
                    break;
                default:
                    setAuthStatus({ type: 'Error', message: 'An error occurred. (' + result.status + ')' });
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
        <>
            <h1 className="mb-4 text-4xl">Login</h1>

            <div className="flex flex-col items-center justify-center w-full max-w-96">
                <TextInput
                    type="email"
                    placeholder="Email"
                    value={email}
                    is_last_position={false}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <TextInput
                    type="password"
                    placeholder="Password"
                    value={password}
                    is_last_position={true}
                    onChange={(e) => setPassword(e.target.value)}
                    onKeyDown={handleKeyDownLogin}
                />
                <Loginbutton content={authStatus.type === 'Loading' ? 'Logging in...' : 'Login'} handleLogin={handleLogin} />
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
        </>
    );
};

export default Login;
