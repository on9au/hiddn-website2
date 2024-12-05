import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const DeleteProfile: React.FC = () => {
    const [isCheckboxChecked, setIsCheckboxChecked] = useState(false);
    const [secondsRemaining, setSecondsRemaining] = useState(10);
    const [isButtonEnabled, setIsButtonEnabled] = useState(false);
    const [isDeleting, setIsDeleting] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    // Countdown timer logic
    useEffect(() => {
        let timer: NodeJS.Timeout;
        if (isCheckboxChecked && secondsRemaining > 0) {
            timer = setInterval(() => {
                setSecondsRemaining((prevSeconds) => prevSeconds - 1);
            }, 1000);
        } else if (secondsRemaining <= 0) {
            setIsButtonEnabled(true);
        }

        return () => {
            if (timer) clearInterval(timer);
        };
    }, [isCheckboxChecked, secondsRemaining]);

    useEffect(() => {
        document.title = 'Delete Account - HiddN';
    })

    // Handle checkbox change
    const handleCheckboxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setIsCheckboxChecked(e.target.checked);
        if (e.target.checked) {
            setSecondsRemaining(10);
            setIsButtonEnabled(false);
        } else {
            setSecondsRemaining(10);
            setIsButtonEnabled(false);
        }
    };

    // Handle account deletion
    const handleDeleteAccount = async () => {
        setIsDeleting(true);
        setError(null);
        try {
            await axios.delete('/api/delete_account', {
                withCredentials: true,
            });
            // Redirect after successful deletion
            navigate('/goodbye');
        } catch (err) {
            setIsDeleting(false);
            setError('Failed to delete account. Please try again later.');
            console.error('Error deleting account:', err);
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Delete Account</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    <h1 className="mb-4 text-2xl font-semibold text-red-500">Before you delete your account:</h1>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        Deleting your account is <span className="font-semibold">permanent</span> and cannot be undone.
                    </p>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        Deleting your account will remove:
                    </p>
                    <ul className="mb-4 ml-5 text-gray-700 list-disc dark:text-gray-300">
                        <li>Your profile information</li>
                        <li>Your settings and preferences</li>
                        <li>Your subscription details</li>
                        <li>All associated data with your account</li>
                    </ul>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        What will not be removed:
                    </p>
                    <ul className="mb-4 ml-5 text-gray-700 list-disc dark:text-gray-300">
                        <li>Transactions/orders history (including your email)</li>
                        <li>Support tickets (not including your email)</li>
                        <li>Any data that is required to be stored for legal reasons</li>
                    </ul>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        <span className="underline">All active subscriptions will be <span className="font-semibold text-red-500">cancelled</span> immediately</span> and you will lose access to the service.
                    </p>
                    <p className="mb-4 font-semibold text-red-500">
                        You will not be refunded for any remaining time on your subscription.
                    </p>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        If you're sure you want to proceed, please confirm below.
                    </p>
                    
                    <div className="flex items-center mb-4">
                        <input
                            id="confirmation-checkbox"
                            type="checkbox"
                            className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                            checked={isCheckboxChecked}
                            onChange={handleCheckboxChange}
                        />
                        <label htmlFor="confirmation-checkbox" className="ml-2 text-gray-700 dark:text-gray-300">
                            I have read and understand what will happen to my account.
                        </label>
                    </div>
                    {isCheckboxChecked && secondsRemaining > 0 && (
                        <p className="mb-4 text-sm text-red-500">
                            You can delete your account in {secondsRemaining} second{secondsRemaining !== 1 && 's'}.
                        </p>
                    )}
                    {error && <p className="mb-4 text-red-500">{error}</p>}
                    <button
                        className={`px-4 py-2 text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none ${(!isButtonEnabled || isDeleting) && 'opacity-50 cursor-not-allowed'
                            }`}
                        onClick={handleDeleteAccount}
                        disabled={!isButtonEnabled || isDeleting}
                    >
                        {isDeleting ? 'Deleting...' : 'Delete My Account'}
                    </button>
                </div>
            </div>
        </div>
    );
};

export default DeleteProfile;
