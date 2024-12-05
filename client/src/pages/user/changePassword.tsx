import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import zxcvbn from 'zxcvbn';
import { ChangePasswordPayload } from '../../bindings';

const ChangePassword: React.FC = () => {
    const [currentPassword, setCurrentPassword] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [successMessage, setSuccessMessage] = useState<string | null>(null);
    const [passwordSuggestions, setPasswordSuggestions] = useState<string[]>([]);
    const [passwordStrength, setPasswordStrength] = useState<number>(0);
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Change Password - HiddN';
    }, []);

    const validateForm = () => {
        if (!currentPassword || !newPassword || !confirmPassword) {
            setError('Please fill out all fields.');
            return false;
        }
        if (newPassword !== confirmPassword) {
            setError('New passwords do not match.');
            return false;
        }
        if (passwordStrength < 3) {
            setError('New password is too weak.');
            return false;
        }
        return true;
    };

    const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const newPassword = e.target.value;
        setNewPassword(newPassword);

        const result = zxcvbn(newPassword);
        setPasswordStrength(result.score);
        if (result.score < 3) {
            setPasswordSuggestions(result.feedback.suggestions);
        } else {
            setPasswordSuggestions([]);
        }
    };

    const handleChangePassword = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setSuccessMessage(null);

        if (!validateForm()) {
            return;
        }

        setIsSubmitting(true);

        const body: ChangePasswordPayload = {
            old_password: currentPassword,
            new_password: newPassword,
            confirm_password: confirmPassword,
        }

        try {
            await axios.post(
                '/api/change_password',
                body,
                { withCredentials: true }
            );
            setSuccessMessage('Your password has been changed successfully. Logging you out in 3 seconds...');
            setCurrentPassword('');
            setNewPassword('');
            setConfirmPassword('');
            setTimeout(() => {
                navigate('/logout');
            }, 3000);
            // Clear form fields
        } catch (err) {
            setIsSubmitting(false);
            if (axios.isAxiosError(err) && err.response) {
                if (err.response.status === 401) {
                    navigate('/logout');
                } else {
                    setError(err.response.data.message || 'Failed to change password. Please try again.');
                }
            } else {
                setError('An unexpected error occurred. Please try again.');
            }
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Change Password</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {error && <p className="mb-4 text-red-500">{error}</p>}
                    {successMessage && <p className="mb-4 text-green-500">{successMessage}</p>}
                    <form onSubmit={handleChangePassword}>
                        <div className="mb-4">
                            <label
                                htmlFor="current-password"
                                className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
                            >
                                Current Password
                            </label>
                            <input
                                id="current-password"
                                type="password"
                                className="w-full px-3 py-2 text-gray-700 bg-gray-200 border rounded-md focus:outline-none focus:ring focus:ring-hiddn-500 dark:bg-gray-700 dark:text-gray-300"
                                value={currentPassword}
                                onChange={(e) => setCurrentPassword(e.target.value)}
                                required
                            />
                        </div>
                        <div className="mb-4">
                            <label
                                htmlFor="new-password"
                                className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
                            >
                                New Password
                            </label>
                            <input
                                id="new-password"
                                type="password"
                                className="w-full px-3 py-2 text-gray-700 bg-gray-200 border rounded-md focus:outline-none focus:ring focus:ring-hiddn-500 dark:bg-gray-700 dark:text-gray-300"
                                value={newPassword}
                                onChange={handlePasswordChange}
                                required
                                minLength={8}
                            />
                        </div>
                        <div className="mb-6">
                            <label
                                htmlFor="confirm-password"
                                className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300"
                            >
                                Confirm New Password
                            </label>
                            <input
                                id="confirm-password"
                                type="password"
                                className="w-full px-3 py-2 text-gray-700 bg-gray-200 border rounded-md focus:outline-none focus:ring focus:ring-hiddn-500 dark:bg-gray-700 dark:text-gray-300"
                                value={confirmPassword}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                                required
                                minLength={8}
                            />
                        </div>
                        {passwordSuggestions.length > 0 && newPassword.length > 0 && (
                            <div className="mb-4 text-yellow-500">
                                <ul>
                                    Your password is too weak. Suggestions:
                                    {passwordSuggestions.map((suggestion, index) => (
                                        <li key={index}>{suggestion}</li>
                                    ))}
                                    {newPassword.length < 9 && (
                                        <li>Password must be 8 or more characters long.</li>
                                    )}
                                </ul>
                            </div>
                        )}
                        <button
                            type="submit"
                            className={`w-full px-4 py-2 text-white bg-hiddn-500 rounded-md hover:bg-hiddn-600 focus:outline-none ${isSubmitting ? 'opacity-50 cursor-not-allowed' : ''
                                }`}
                            disabled={isSubmitting}
                        >
                            {isSubmitting ? 'Changing Password...' : 'Change Password'}
                        </button>
                    </form>
                </div>
            </div>
        </div>
    );
};

export default ChangePassword;