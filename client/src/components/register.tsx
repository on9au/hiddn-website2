import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthStatus, EmailVerifyStatus, RegisterPayload } from '../auth';
import FrontpageHeader from './frontpageheader';
import CommonLink from './commonlink';
import ICON from '../assets/hiddn_icon.svg';
import { VerifyEmailPayload } from '../bindings';

const apiURL: string = import.meta.env.VITE_API_URL;

const Register: React.FC = () => {
    const navigate = useNavigate();

    const [email, setEmail] = useState<string>('');
    const [emailVerificationCode, setEmailVerificationCode] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [confirmPassword, setConfirmPassword] = useState<string>('');
    const [inviteCode, setInviteCode] = useState<string>('');
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

        const result = await fetch(`${apiURL}/verify_email_register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(payload),
        }).then((response) => {
            return response;
        });

        if (result.type === 'Success') {
            setVerifyStatus({ type: 'Sent' });
            // Start a countdown for 60 seconds
            for (let i = 60; i > 0; i--) {
                setVerifyTimeout(i);
                await new Promise(res => setTimeout(res, 1000));
            }
            setVerifyStatus({ type: 'Idle' });
        } else {
            setVerifyStatus({ type: 'Error', message: result.message });
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

        setAuthStatus({ type: 'Loading' });

        const payload: RegisterPayload = {
            email,
            email_verification_code: emailVerificationCode,
            password,
            confirm_password: confirmPassword,
            invite_code: inviteCode,
        };

        const result = await registerUser(payload);

        if (result.type === 'Token') {
            setAuthStatus({ type: 'Success' });
            navigate('/dashboard');
        } else {
            setAuthStatus({ type: 'Error', message: result.message });
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
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100">
            <FrontpageHeader icon={ICON} title="HiddN" />

            <h1 className="mb-4 text-4xl">Register</h1>

            <div className="flex flex-col items-center justify-center w-full max-w-96">
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="email"
                    placeholder="Email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <div className="relative w-full">
                    <input
                        className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
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
                        {verifyStatus.type === 'Idle' ? 'Send' : verifyStatus.type === 'Loading' ? '...' : verifyTimeout.toString()}
                    </button>
                </div>
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="password"
                    placeholder="Password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
                <input
                    className="w-full px-4 py-2 mb-2 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="password"
                    placeholder="Confirm Password"
                    value={confirmPassword}
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    onKeyDown={handleKeyDownRegister}
                />
                <input
                    className="w-full px-4 py-2 mb-4 border border-gray-300 outline-none hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl"
                    type="text"
                    placeholder="Invite Code (Optional)"
                    value={inviteCode}
                    onChange={(e) => setInviteCode(e.target.value)}
                    onKeyDown={handleKeyDownRegister}
                />
                <button
                    className="w-full px-4 py-2 mb-4 text-white bg-hiddn-500 hover:bg-hiddn-400 rounded-xl"
                    onClick={handleRegister}
                >
                    {authStatus.type === 'Loading' ? 'Registering account...' : 'Register'}
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
                    <CommonLink to="/login" text="Login" />
                    <CommonLink to="/forgot" text="Forgot password" />
                </div>
            </div>
        </div>
    );
};

export default Register;
