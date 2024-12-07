import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';
import { UserProfilePayload } from '../../bindings';
import { Switch } from '@headlessui/react'; // Assuming you're using Headless UI for switches

type FetchUserStatusEnum =
    | { status: 'loading' }
    | { status: 'success' }
    | { status: 'error'; message: string };

const SkeletonProfile: React.FC = () => {
    return (
        <div className="space-y-8 animate-pulse">
            <div className="w-1/3 h-8 bg-gray-300 rounded"></div>
            <div className="space-y-4">
                <div className="w-1/4 h-6 bg-gray-300 rounded"></div>
                <div className="w-1/2 h-6 bg-gray-300 rounded"></div>
                <div className="w-1/3 h-6 bg-gray-300 rounded"></div>
            </div>
            <div className="space-y-4">
                <div className="w-1/4 h-6 bg-gray-300 rounded"></div>
                <div className="w-1/2 h-6 bg-gray-300 rounded"></div>
                <div className="w-1/3 h-6 bg-gray-300 rounded"></div>
            </div>
        </div>
    );
};

const formatUnixTimestamp = (timestamp: number): string => {
    return new Date(timestamp * 1000).toLocaleString(undefined, { timeZoneName: 'short' });
}

const Profile: React.FC = () => {
    const [userProfile, setUserProfile] = useState<UserProfilePayload | null>(null);
    const [fetchServerStatus, setFetchServerStatus] = useState<FetchUserStatusEnum>({ status: 'loading' });
    const [emailExpReminder, setEmailExpReminder] = useState<boolean>(false);
    const [emailDataReminder, setEmailDataReminder] = useState<boolean>(false);
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Hiddn | Profile';

        const fetchUserProfile = async () => {
            try {
                const response = await axios.get<UserProfilePayload>('/api/me', {
                    withCredentials: true,
                });
                setUserProfile(response.data);
                setEmailExpReminder(response.data.email_expiration_reminder);
                setEmailDataReminder(response.data.email_data_reminder);
                setFetchServerStatus({ status: 'success' });
            } catch (error) {
                if (axios.isAxiosError(error)) {
                    if (error.response) {
                        if (error.response.status === 401) {
                            setFetchServerStatus({ status: 'error', message: 'Unauthorized. Please log in.' });
                            navigate('/logout');
                        } else {
                            setFetchServerStatus({ status: 'error', message: error.response.statusText });
                        }
                    } else {
                        setFetchServerStatus({
                            status: 'error',
                            message: 'Failed to fetch user profile. Error: ' + error.message,
                        });
                    }
                } else {
                    setFetchServerStatus({
                        status: 'error',
                        message: 'An unexpected error occurred: ' + error,
                    });
                }
            }
        };

        fetchUserProfile();
    }, [navigate]);

    // Handler functions
    const handleChangePassword = () => {
        navigate('/user/change-password');
    };

    const handleResetSubscriptionURL = async () => {
        // Ask the user to confirm
        if (!window.confirm('Are you sure you want to reset your subscription URL? You will need to update your subscription URL for all of your devices.')) {
            return;
        }

        try {
            await axios.post(
                '/api/reset_subscription_url',
                {},
                { withCredentials: true }
            );
            alert('Subscription URL has been reset.');
        } catch (error) {
            console.error('Failed to reset subscription URL:', error);
            alert('Failed to reset subscription URL.');
        }
    };

    const handleToggleExpReminder = async () => {
        try {
            const newValue = !emailExpReminder;
            await axios.post(
                '/api/update_settings',
                { email_expiration_reminder: newValue },
                { withCredentials: true }
            );
            setEmailExpReminder(newValue);
        } catch (error) {
            console.error('Failed to update expiration reminder setting:', error);
            alert('Failed to update setting.');
        }
    };

    const handleToggleDataReminder = async () => {
        try {
            const newValue = !emailDataReminder;
            await axios.post(
                '/api/update_settings',
                { email_data_reminder: newValue },
                { withCredentials: true }
            );
            setEmailDataReminder(newValue);
        } catch (error) {
            console.error('Failed to update data reminder setting:', error);
            alert('Failed to update setting.');
        }
    };

    const handleDeleteAccount = () => {
        navigate('/user/delete-account');
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Profile</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {fetchServerStatus.status === 'loading' ? (
                        <SkeletonProfile />
                    ) : fetchServerStatus.status === 'error' ? (
                        <p className="text-red-500">{fetchServerStatus.message}</p>
                    ) : userProfile ? (
                        <div className="space-y-8">
                            {/* User Details */}
                            <div className="flex flex-col lg:flex-row lg:space-x-6">
                                <div className="flex-1">
                                    <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                        {userProfile.email}
                                    </h2>
                                    <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Email:</strong> {userProfile.email}
                                    </p>
                                    <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Joined:</strong>{' '}
                                        {formatUnixTimestamp(userProfile.created_at)}
                                    </p>
                                    <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Last Updated:</strong>{' '}
                                        {formatUnixTimestamp(userProfile.updated_at)}
                                    </p>
                                </div>
                            </div>
                            {/* Settings Section */}
                            <div className="mt-8">
                                <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    Settings
                                </h3>
                                <div className="space-y-6">
                                    {/* Change Password */}
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-base text-gray-700 dark:text-gray-300">
                                                Change Password
                                            </p>
                                            <p className="text-sm text-gray-600 dark:text-gray-400">
                                                Update your account password.
                                            </p>
                                        </div>
                                        <button
                                            className="px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none"
                                            onClick={handleChangePassword}
                                        >
                                            Change
                                        </button>
                                    </div>
                                    {/* Reset Subscription URL */}
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-base text-gray-700 dark:text-gray-300">
                                                Reset Subscription URL
                                            </p>
                                            <p className="text-sm text-gray-600 dark:text-gray-400">
                                                Generate a new subscription URL for your account.
                                            </p>
                                        </div>
                                        <button
                                            className="px-4 py-2 text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none"
                                            onClick={handleResetSubscriptionURL}
                                        >
                                            Reset
                                        </button>
                                    </div>
                                    {/* Expiration Reminder to Email */}
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-base text-gray-700 dark:text-gray-300">
                                                Expiration Reminder Emails
                                            </p>
                                            <p className="text-sm text-gray-600 dark:text-gray-400">
                                                Receive email reminders when your plan is about to expire.
                                            </p>
                                        </div>
                                        <Switch
                                            checked={emailExpReminder}
                                            onChange={handleToggleExpReminder}
                                            className={`${emailExpReminder ? 'bg-hiddn-500' : 'bg-gray-200 dark:bg-gray-700'
                                                } relative inline-flex h-6 w-11 items-center rounded-full`}
                                        >
                                            <span
                                                className={`${emailExpReminder ? 'translate-x-6' : 'translate-x-1'
                                                    } inline-block h-4 w-4 transform bg-white rounded-full transition-transform`}
                                            />
                                        </Switch>
                                    </div>
                                    {/* Data Limit Reminder to Email */}
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-base text-gray-700 dark:text-gray-300">
                                                Data Limit Reminder Emails
                                            </p>
                                            <p className="text-sm text-gray-600 dark:text-gray-400">
                                                Receive email alerts when you are close to your data limit.
                                            </p>
                                        </div>
                                        <Switch
                                            checked={emailDataReminder}
                                            onChange={handleToggleDataReminder}
                                            className={`${emailDataReminder ? 'bg-hiddn-500' : 'bg-gray-200 dark:bg-gray-700'
                                                } relative inline-flex h-6 w-11 items-center rounded-full`}
                                        >
                                            <span
                                                className={`${emailDataReminder ? 'translate-x-6' : 'translate-x-1'
                                                    } inline-block h-4 w-4 transform bg-white rounded-full transition-transform`}
                                            />
                                        </Switch>
                                    </div>
                                    {/* Delete Account */}
                                    <div className="flex items-center justify-between">
                                        <div>
                                            <p className="text-base text-gray-700 dark:text-gray-300">
                                                Delete Account
                                            </p>
                                            <p className="text-sm text-gray-600 dark:text-gray-400">
                                                Permanently delete your account and all associated data.
                                            </p>
                                        </div>
                                        <button
                                            className="px-4 py-2 text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none"
                                            onClick={handleDeleteAccount}
                                        >
                                            Delete
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    ) : (
                        <p className="text-gray-700 dark:text-gray-300">No user profile data available.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Profile;