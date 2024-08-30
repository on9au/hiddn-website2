import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthStatus, EmailVerifyStatus } from '../auth';
import FrontpageHeader from '../components/frontpageheader';
import CommonLink from '../components/commonlink';
import ICON from '../assets/favicon.svg';
import { ForgotPasswordPayload, VerifyEmailPayload } from '../bindings';

// const apiURL: string = import.meta.env.VITE_API_URL;

const Forgot: React.FC = () => {
    const navigate = useNavigate();

    const [email, setEmail] = useState<string>('');
    const [emailVerificationCode, setEmailVerificationCode] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [confirmPassword, setConfirmPassword] = useState<string>('');
    const [authStatus, setAuthStatus] = useState<AuthStatus>({ type: 'Idle' });
    const [verifyStatus, setVerifyStatus] = useState<EmailVerifyStatus>({ type: 'Idle' });
    const [verifyTimeout, setVerifyTimeout] = useState<number>(0);

    const handleSendCode = async () => {
        if (email === '' || !email.includes('@')) {
            setVerifyStatus({ type: 'Error', message: 'Please enter a valid email.' });
            return;
        }

        setVerifyStatus({ type: 'Loading' });

        const payload: VerifyEmailPayload = { email };

        const result = await fetch(`api/verify_email`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        }).then((response) => {
            return response;
        });

        if (result.status === 200) {
            setVerifyStatus({ type: 'Sent' });
            // Start a countdown for 60 seconds
            for (let i = 60; i > 0; i--) {
                setVerifyTimeout(i);
                await new Promise(res => setTimeout(res, 1000));
            }
            setVerifyStatus({ type: 'Idle' });
        } else {
            setVerifyStatus({ type: 'Error', message: result.statusText });
        }
    };

    const handleRegister = async () => {
        if (email === '' || !email.includes('@')) {
            setAuthStatus({ type: 'Error', message: 'Please enter a valid email.' });
            return;
        }

        if (password !== confirmPassword) {
            setAuthStatus({ type: 'Error', message: 'Passwords do not match.' });
            return;
        }

        // Verify password strength
        // Password must be at least 8 characters long, contain at least one uppercase letter, one lowercase letter, and one number.
        // Check length first
        if (password.length < 8) {
            setAuthStatus({ type: 'Error', message: 'Password must be at least 8 characters long.' });
            return;
        }
        // Check for uppercase letter
        if (!/[A-Z]/.test(password)) {
            setAuthStatus({ type: 'Error', message: 'Password must contain at least one uppercase letter.' });
            return;
        }
        // Check for lowercase letter
        if (!/[a-z]/.test(password)) {
            setAuthStatus({ type: 'Error', message: 'Password must contain at least one lowercase letter.' });
            return;
        }
        // Check for number
        if (!/[0-9]/.test(password)) {
            setAuthStatus({ type: 'Error', message: 'Password must contain at least one number.' });
            return;
        }

        setAuthStatus({ type: 'Loading' });

        const payload: ForgotPasswordPayload = {
            email,
            email_verification_code: emailVerificationCode,
            password,
            confirm_password: confirmPassword,
        };

        const result = await fetch(`api/forgot_password`, {
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
                // Automatically log in user
                const login_result = await fetch(`api/login_user`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ email, password }),
                }).then((response) => {
                    return response;
                });
                switch (login_result.status) {
                    case 200:
                        setAuthStatus({ type: 'Success' });
                        navigate('/user/dashboard');
                        break;
                    default:
                        setAuthStatus({ type: 'Error', message: 'Account password reset. Automatic login failed. Try logging in yourself. Redirecting in 5 seconds...' });
                        // wait for 5 seconds before redirecting to login page
                        await new Promise(res => setTimeout(res, 5000));
                        navigate('/login');
                }
                break;
            case 404:
                setAuthStatus({ type: 'Error', message: 'User does not exist.' });
                break;
            case 403:
                setAuthStatus({ type: 'Error', message: 'Incorrect email verification code.' });
                break;
            case 409:
                setAuthStatus({ type: 'Error', message: 'Server failed to validate passwords. Try again.' });
                break;
            default:
                setAuthStatus({ type: 'Error', message: 'An error occurred. (' + result.status + ')' });
        }
    };

    const handleKeyDownRegister = async (e: React.KeyboardEvent) => {
        if (password === '') {
            return;
        }
        if (e.key === 'Enter') {
            await handleRegister();
        }
    };

    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100 dark:bg-gray-900 dark:text-white">
            <FrontpageHeader icon={ICON} title="HiddN" />

            <h1 className="mb-4 text-4xl">Reset Password</h1>

            <div className="flex flex-col items-center justify-center w-full max-w-96">
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none dark:border-gray-700 dark:bg-gray-800 dark:hover:border-hiddn-600 hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="email"
                    placeholder="Email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <div className="relative w-full">
                    <input
                        className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none dark:border-gray-700 dark:bg-gray-800 dark:hover:border-hiddn-600 hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                        type="text"
                        placeholder="Verification Code"
                        maxLength={20}
                        value={emailVerificationCode}
                        onChange={(e) => setEmailVerificationCode(e.target.value)}
                    />
                    <button
                        className={`absolute right-0 w-20 px-4 py-2 text-white border rounded-xl ${verifyStatus.type === 'Idle' || verifyStatus.type === 'Error' ? 'bg-hiddn-500 hover:bg-hiddn-400 border-hiddn-500 hover:border-hiddn-400' : 'bg-gray-400 border-gray-400 cursor-not-allowed'}`}
                        onClick={async () => {
                            if (verifyStatus.type === 'Idle' || verifyStatus.type === 'Error') {
                                await handleSendCode();
                            }
                        }}
                        disabled={verifyStatus.type !== 'Idle' && verifyStatus.type !== 'Error'}
                    >
                        {verifyStatus.type === 'Idle' ? 'Send' : verifyStatus.type === 'Error' ? 'Send' : verifyStatus.type === 'Loading' ? '...' : verifyTimeout.toString()}
                    </button>
                </div>
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none dark:border-gray-700 dark:bg-gray-800 dark:hover:border-hiddn-600 hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="password"
                    placeholder="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none dark:border-gray-700 dark:bg-gray-800 dark:hover:border-hiddn-600 hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="password"
                    placeholder="Confirm Password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    onKeyDown={handleKeyDownRegister}
                />
                <button
                    className="w-full px-4 py-2 mb-4 text-white bg-hiddn-500 hover:bg-hiddn-400 dark:hover:bg-hiddn-600 rounded-xl"
                    onClick={handleRegister}
                >
                    {authStatus.type === 'Loading' ? 'Resetting account...' : 'Reset Password'}
                </button>
                {authStatus.type === 'Error' && (
                    <div className="mb-4 text-red-500">
                        {authStatus.message}
                    </div>
                )}
                {authStatus.type === 'Success' && (
                    <div className="mb-4 text-green-500">
                        Success!
                    </div>
                )}
                {verifyStatus.type === 'Error' && (
                    <div className="mb-4 text-red-500">
                        {verifyStatus.message}
                    </div>
                )}
                <div className="flex flex-row space-x-4">
                    <CommonLink to="/login" text="Back to Login" />
                </div>
            </div>
        </div>
    );
};

export default Forgot;
