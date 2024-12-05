import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const ChangePassword: React.FC = () => {
    const [currentPassword, setCurrentPassword] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [successMessage, setSuccessMessage] = useState<string | null>(null);
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
        if (newPassword.length < 8) {
            setError('New password must be at least 8 characters long.');
            return false;
        }
        return true;
    };

    const handleChangePassword = async (e: React.FormEvent) => {
        e.preventDefault();
        setError(null);
        setSuccessMessage(null);

        if (!validateForm()) {
            return;
        }

        setIsSubmitting(true);

        try {
            await axios.post(
                '/api/change_password',
                {
                    current_password: currentPassword,
                    new_password: newPassword,
                },
                { withCredentials: true }
            );
            setSuccessMessage('Your password has been changed successfully.');
            // Clear form fields
            setCurrentPassword('');
            setNewPassword('');
            setConfirmPassword('');
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
                                onChange={(e) => setNewPassword(e.target.value)}
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
