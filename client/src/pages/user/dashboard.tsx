import React, { useEffect, useState } from 'react';
import { FaArrowUp, FaArrowRight } from 'react-icons/fa';
import axios from 'axios';
import { PlanDetailsPayload } from '../../bindings';
import { useNavigate } from 'react-router-dom';

const Dashboard: React.FC = () => {
    const [planDetails, setPlanDetails] = useState<PlanDetailsPayload | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchPlanDetails = async () => {
            try {
                const response = await axios.get<PlanDetailsPayload>('/api/plan_details', {
                    withCredentials: true,
                });
                setPlanDetails(response.data);
                setError(null);
            } catch (err) {
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 401) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        }
                    }
                }
                console.error('Failed to load plan details. Error:', err);
                setError('Failed to load plan details.');
            } finally {
                setLoading(false);
            }
        };

        document.title = 'Dashboard - HiddN';
        fetchPlanDetails();
    }, [navigate]);

    const currentTime = new Date();
    let greeting = '';

    if (currentTime.getHours() < 12) {
        greeting = 'Good morning';
    } else if (currentTime.getHours() < 18) {
        greeting = 'Good afternoon';
    } else {
        greeting = 'Good evening';
    }

    // Calculate data usage percentage
    const dataUsagePercentage = planDetails
        ? Math.min((planDetails.dataUsed / planDetails.dataLimit) * 100, 100)
        : 0;

    return (
        <div className="flex flex-col items-center min-h-screen pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">{greeting}.</h1>
            </span>
            <div className="container mx-auto">
                {loading ? (
                    <div className="flex items-center justify-center h-full">
                        <div
                            className="inline-block w-12 h-12 border-4 border-current border-blue-500 border-solid rounded-full animate-spin border-r-transparent"
                            role="status"
                        >
                            <span className="sr-only">Loading...</span>
                        </div>
                    </div>
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : planDetails ? (
                    <div className="flex flex-col lg:flex-row lg:space-x-6">
                        {/* Plan Details Card */}
                        <div className="w-full mb-6 lg:w-1/2 lg:mb-0">
                            <div className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    Your Plan
                                </h3>
                                <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                    <strong>Expiration:</strong> {planDetails.expiration}
                                </p>
                                <p className="mb-4 text-base text-gray-700 dark:text-gray-300">
                                    <strong>Status:</strong> {planDetails.status}
                                </p>
                                {/* Data Usage Progress Bar */}
                                <div className="mb-4">
                                    <p className="mb-1 text-base text-gray-700 dark:text-gray-300">
                                        Data Usage: {planDetails.dataUsed}GB / {planDetails.dataLimit}GB
                                    </p>
                                    <div className="w-full h-4 bg-gray-300 rounded-full dark:bg-gray-700">
                                        <div
                                            className="h-4 rounded-full bg-hiddn-500"
                                            style={{ width: `${dataUsagePercentage}%` }}
                                        ></div>
                                    </div>
                                    <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                                        {dataUsagePercentage.toFixed(2)}% used
                                    </p>
                                </div>
                                {/* Quick Actions */}
                                <div className="flex space-x-4">
                                    <button className="flex items-center px-4 py-2 text-white align-middle bg-green-500 rounded-md hover:bg-green-600 focus:outline-none">
                                        <span>Upgrade Plan</span> <FaArrowUp className="inline-block ml-2" />
                                    </button>
                                    <button className="flex items-center px-4 py-2 text-white align-middle bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none">
                                        <span>View Transactions</span> <FaArrowRight className="inline-block ml-2" />
                                    </button>
                                </div>
                            </div>
                        </div>
                        {/* Recent Activity or Notifications */}
                        <div className="w-full lg:w-1/2">
                            <div className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    Recent Activity
                                </h3>
                                {/* Sample Activity List */}
                                <ul className="space-y-4">
                                    <li className="text-gray-700 dark:text-gray-300">
                                        • Connected to Melbourne server on {currentTime.toLocaleDateString()}
                                    </li>
                                    <li className="text-gray-700 dark:text-gray-300">
                                        • Data usage updated: {planDetails.dataUsed}GB used
                                    </li>
                                    <li className="text-gray-700 dark:text-gray-300">
                                        • Plan renewed on {planDetails.expiration}
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                ) : (
                    <p className="text-gray-700 dark:text-gray-300">No plan details available.</p>
                )}
            </div>
        </div>
    );
};

export default Dashboard;
