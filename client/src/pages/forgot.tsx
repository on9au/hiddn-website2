import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import zxcvbn from 'zxcvbn';
import { AuthStatus, EmailVerifyStatus } from '../auth';
import CommonLink from '../components/commonlink';
import { ForgotPasswordPayload, RequestCodePayload, PasswordFeedbackPayload } from '../bindings';
import Loginbutton from '../components/loginbutton';
import TextInput from '../components/logintextinput';
import VerificationInput from '../components/loginpageverificationinput';

// const apiURL: string = import.meta.env.VITE_API_URL;

const Forgot: React.FC = () => {
    useEffect(() => { document.title = 'Forgot - HiddN'; });

    const navigate = useNavigate();

    const [email, setEmail] = useState<string>('');
    const [emailVerificationCode, setEmailVerificationCode] = useState<string>('');
    const [password, setPassword] = useState<string>('');
    const [confirmPassword, setConfirmPassword] = useState<string>('');
    const [authStatus, setAuthStatus] = useState<AuthStatus>({ type: 'Idle' });
    const [verifyStatus, setVerifyStatus] = useState<EmailVerifyStatus>({ type: 'Idle' });
    const [verifyTimeout, setVerifyTimeout] = useState<number>(0);
    const [passwordSuggestions, setPasswordSuggestions] = useState<string[]>([]);
    const [passwordStrength, setPasswordStrength] = useState<number>(0);

    const handleSendCode = async () => {
        if (email === '' || !email.includes('@')) {
            setVerifyStatus({ type: 'Error', message: 'Please enter a valid email.' });
            return;
        }

        setVerifyStatus({ type: 'Loading' });

        const payload: RequestCodePayload = { email };

        const result = await fetch(`api/request_code`, {
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

    const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newPassword = e.target.value;
        setConfirmPassword(newPassword);

        const result = zxcvbn(newPassword);
        setPasswordStrength(result.score);
        setPasswordSuggestions(result.feedback.suggestions);
        if (passwordStrength < 3) {
            setPasswordSuggestions(result.feedback.suggestions);
        } else {
            setPasswordSuggestions([]);
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

        if (passwordStrength < 3) {
            setAuthStatus({ type: 'Error', message: 'Password is too weak.' });
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
                {
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
                }
            case 404:
                setAuthStatus({ type: 'Error', message: 'User does not exist.' });
                break;
            case 403:
                setAuthStatus({ type: 'Error', message: 'Incorrect email verification code.' });
                break;
            case 409: {
                const feedback: PasswordFeedbackPayload = await result.json();
                setAuthStatus({ type: 'Error', message: 'Password is too weak: ' + feedback.warning || 'Password validation failed.' });
                setPasswordSuggestions(feedback.suggestions);
                break;
            }
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
        <>
            <h1 className="mb-4 text-4xl">Reset Password</h1>

            <div className="flex flex-col items-center justify-center w-full max-w-96">
                <TextInput
                    type="email"
                    placeholder="Email"
                    value={email}
                    is_last_position={false}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <VerificationInput
                    value={emailVerificationCode}
                    onChange={(e) => setEmailVerificationCode(e.target.value)}
                    onSendCode={handleSendCode}
                    verifyStatus={verifyStatus}
                    verifyTimeout={verifyTimeout}
                />
                <TextInput
                    type="password"
                    placeholder="Password"
                    value={password}
                    is_last_position={false}
                    onChange={(e) => setPassword(e.target.value)}
                />
                <TextInput
                    type="password"
                    placeholder="Confirm Password"
                    value={confirmPassword}
                    is_last_position={true}
                    onChange={handlePasswordChange}
                    onKeyDown={handleKeyDownRegister}
                />
                <Loginbutton content={authStatus.type === 'Loading' ? 'Resetting account...' : 'Reset Password'} handleLogin={handleRegister} />
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
                {passwordSuggestions.length > 0 && confirmPassword.length > 0 && (
                    <div className="mb-4 text-yellow-500">
                        <ul>
                            Your password is too weak. Suggestions:
                            {passwordSuggestions.map((suggestion, index) => (
                                <li key={index}>{suggestion}</li>
                            ))}
                            {confirmPassword.length < 9 && (
                                <li>Password must be 8 or more characters long.</li>
                            )}
                        </ul>
                    </div>
                )}
                <div className="flex flex-row space-x-4">
                    <CommonLink to="/login" text="Back to Login" />
                </div>
            </div>
        </>
    );
};

export default Forgot;