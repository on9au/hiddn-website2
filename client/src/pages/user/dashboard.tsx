import React, { useEffect, useState } from 'react';
import { FaArrowUp, FaArrowRight } from 'react-icons/fa';
import axios from 'axios';
import ReactMarkdown from 'react-markdown';
import { useNavigate } from 'react-router-dom';
import { DownloadIcon } from '@primer/octicons-react';
import { PlanDetails } from '../../bindings/PlanDetails';
import { Announcement } from '../../bindings/Announcement';

const SkeletonDashboard: React.FC = () => {
    return (
        <div className="space-y-8 animate-pulse">
            <div className="w-1/3 h-8 bg-gray-300 rounded"></div>
            <div className="flex flex-col lg:flex-row lg:space-x-6">
                <div className="w-full mb-6 lg:w-1/2 lg:mb-0">
                    <div className="p-6 space-y-4 bg-white rounded-lg shadow-md dark:bg-gray-800">
                        <div className="w-1/2 h-6 bg-gray-300 rounded"></div>
                        <div className="w-1/4 h-4 bg-gray-300 rounded"></div>
                        <div className="w-1/4 h-4 bg-gray-300 rounded"></div>
                        <div className="w-full h-4 bg-gray-300 rounded"></div>
                        <div className="w-full h-4 bg-gray-300 rounded"></div>
                        <div className="w-1/3 h-4 bg-gray-300 rounded"></div>
                    </div>
                    <div className="p-6 mt-6 space-y-4 bg-white rounded-lg shadow-md dark:bg-gray-800">
                        <div className="w-1/2 h-6 bg-gray-300 rounded"></div>
                        <div className="w-full h-4 bg-gray-300 rounded"></div>
                        <div className="w-1/3 h-4 bg-gray-300 rounded"></div>
                    </div>
                </div>
                <div className="w-full lg:w-1/2">
                    <div className="p-6 space-y-4 bg-white rounded-lg shadow-md dark:bg-gray-800">
                        <div className="w-1/2 h-6 bg-gray-300 rounded"></div>
                        <div className="space-y-4">
                            <div className="w-full h-4 bg-gray-300 rounded"></div>
                            <div className="w-full h-4 bg-gray-300 rounded"></div>
                            <div className="w-full h-4 bg-gray-300 rounded"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

const formatBytes = (bytes: bigint, decimals = 2): string => {
    const bytes_n = Number(bytes);
    if (bytes_n === 0) return '0 Bytes';
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    const i = Math.floor(Math.log(bytes_n) / Math.log(k));
    return parseFloat((bytes_n / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

const formatUnixTimestamp = (timestamp: string): string => {
    return new Date(timestamp).toLocaleString(undefined, { timeZoneName: 'short' });
}

const Dashboard: React.FC = () => {
    const [planDetails, setPlanDetails] = useState<PlanDetails | null>(null);
    const [announcements, setAnnouncements] = useState<Announcement[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchPlanDetails = async () => {
            try {
                const response = await axios.get<PlanDetails>('/api/me/plan-details', {
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

        const fetchAnnouncements = async () => {
            try {
                const response = await axios.get('/api/announcements', {
                    withCredentials: true,
                });
                const reversedAnnouncements = response.data.reverse();
                setAnnouncements(reversedAnnouncements);
            } catch (err) {
                console.error('Failed to load announcements. Error:', err);
            }
        };

        document.title = 'Dashboard - HiddN';
        fetchPlanDetails();
        fetchAnnouncements();
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
        ? planDetails.dataLimit ? (Number(planDetails.dataUsed) / Number(planDetails.dataLimit)) * 100 : 0
        : 0;

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">{greeting}.</h1>
            </span>
            <div className="container mx-auto">
                {loading ? (
                    <SkeletonDashboard />
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : (
                    <div className="flex flex-col lg:flex-row lg:space-x-6">
                        {/* Plan Details or Call to Action */}
                        <div className="w-full mb-6 lg:w-1/2 lg:mb-0">
                            {planDetails ? (
                                <div className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                    <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                        Your Plan
                                    </h3>
                                    <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Expiration:</strong> {planDetails.expiration ? formatUnixTimestamp(planDetails.expiration) : "Never"}
                                    </p>
                                    <p className="mb-4 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Status:</strong> {planDetails.status}
                                    </p>
                                    {/* Data Usage Progress Bar */}
                                    <div className="mb-4">
                                        <p className="mb-1 text-base text-gray-700 dark:text-gray-300">
                                            Data Usage: {formatBytes(planDetails.dataUsed)}{planDetails.dataLimit?.toString() && ` / ${formatBytes(planDetails.dataLimit)}`}
                                        </p>
                                        {
                                            Number(planDetails.dataLimit) && (
                                                <>
                                                    <div className="w-full h-4 bg-gray-300 rounded-full dark:bg-gray-700">
                                                        <div
                                                            className={
                                                                dataUsagePercentage >= 80
                                                                    ? "h-4 rounded-full bg-orange-600"
                                                                    : dataUsagePercentage >= 100
                                                                        ? "h-4 rounded-full bg-red-500"
                                                                        : "h-4 rounded-full bg-hiddn-500"
                                                            }
                                                            style={{ width: `${dataUsagePercentage}%` }}
                                                        ></div>
                                                    </div>
                                                    <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
                                                        {dataUsagePercentage.toFixed(2)}% used
                                                    </p>
                                                </>
                                            )
                                        }
                                    </div>
                                    {/* Quick Actions */}
                                    <div className="flex space-x-4">
                                        <button
                                            className="flex items-center px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 focus:outline-none"
                                            onClick={() => navigate('/user/plan')}
                                        >
                                            <span>Upgrade Plan</span> <FaArrowUp className="inline-block ml-2" />
                                        </button>
                                        <button
                                            className="flex items-center px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none"
                                            onClick={() => navigate('/user/transaction')}
                                        >
                                            <span>View Transactions</span> <FaArrowRight className="inline-block ml-2" />
                                        </button>
                                    </div>
                                </div>
                            ) : (
                                <div className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                    <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                        No Active Plan
                                    </h3>
                                    <p className="mb-4 text-base text-gray-700 dark:text-gray-300">
                                        You don't have an active plan at the moment. To enjoy our VPN services, please
                                        consider purchasing a plan that suits your needs.
                                    </p>
                                    <button
                                        className="flex items-center px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 focus:outline-none"
                                        onClick={() => navigate('/user/plan')}
                                    >
                                        <span>View Plans</span> <FaArrowUp className="inline-block ml-2" />
                                    </button>
                                </div>
                            )}
                            {/* Documentation Section */}
                            <div className="p-6 mt-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    {planDetails ? 'Ready to connect?' : 'Download the app'}
                                </h3>
                                <p className="mb-4 text-base text-gray-700 dark:text-gray-300">
                                    Install the HiddN VPN App below.
                                </p>
                                <button
                                    className="flex items-center px-4 py-2 text-white rounded-md bg-hiddn-500 hover:bg-hiddn-600 focus:outline-none"
                                    onClick={() => navigate('/user/documentation')}
                                >
                                    <span>Install App</span> <DownloadIcon className="inline-block ml-2" />
                                </button>
                            </div>
                        </div>
                        {/* Announcements Section */}
                        <div className="w-full lg:w-1/2">
                            <div className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    Announcements
                                </h3>
                                {announcements.length > 0 ? (
                                    <div className="space-y-6">
                                        {announcements.map((announcement) => (
                                            <div key={announcement.id} className="pb-4 border-b">
                                                <h4 className="text-xl font-semibold text-gray-800 dark:text-gray-200">
                                                    {announcement.title}
                                                </h4>
                                                <p className="text-sm text-gray-600 dark:text-gray-400">
                                                    {new Date(announcement.date).toLocaleDateString()}
                                                </p>
                                                <div className="mt-2 prose max-w-none dark:prose-invert">
                                                    <ReactMarkdown>{announcement.content}</ReactMarkdown>
                                                </div>
                                            </div>
                                        ))}
                                    </div>
                                ) : (
                                    <p className="text-gray-700 dark:text-gray-300">
                                        No announcements at this time.
                                    </p>
                                )}
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default Dashboard;