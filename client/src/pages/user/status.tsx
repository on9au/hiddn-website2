import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import { ServerStatusPayload } from '../../bindings';

type FetchServerStatusEnum =
    | { status: 'loading' }
    | { status: 'success' }
    | { status: 'error'; message: string };

const Status: React.FC = () => {
    const [serverStatus, setServerStatus] = useState<ServerStatusPayload[]>([]);
    const [fetchStatus, setFetchStatus] = useState<FetchServerStatusEnum>({ status: 'loading' });
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Server Status - HiddN';

        const fetchServerStatus = async () => {
            try {
                const response = await axios.get<ServerStatusPayload[]>('/api/server_status', {
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    withCredentials: true, // Include cookies for authentication if needed
                });

                setServerStatus(response.data);
                setFetchStatus({ status: 'success' });
            } catch (error) {
                if (axios.isAxiosError(error)) {
                    if (error.response) {
                        if (error.response.status === 401) {
                            setFetchStatus({ status: 'error', message: 'Unauthorized. Please log in.' });
                            navigate('/logout');
                        } else {
                            setFetchStatus({ status: 'error', message: error.response.statusText });
                        }
                    } else {
                        setFetchStatus({
                            status: 'error',
                            message: 'Failed to fetch server status. Please try again later. Error: ' + error.message,
                        });
                    }
                } else {
                    setFetchStatus({
                        status: 'error',
                        message: 'An unexpected error occurred: ' + error,
                    });
                }
            }
        };

        fetchServerStatus();
    }, [navigate]);

    // Function to get status styles
    const getStatusStyle = (status: string) => {
        switch (status.toLowerCase()) {
            case 'online':
                return 'text-green-500';
            case 'offline':
                return 'text-red-500';
            case 'unreachable':
                return 'text-red-500';
            case 'degraded':
                return 'text-orange-500 dark:text-yellow-300';
            case 'maintenance':
                return 'text-orange-500 dark:text-yellow-300';
            default:
                return 'text-gray-700 dark:text-gray-300';
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Server Status</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {fetchStatus.status === 'loading' ? (
                        <div className="flex items-center justify-center h-full">
                            <div
                                className="inline-block w-12 h-12 border-4 border-current border-blue-500 border-solid rounded-full animate-spin border-r-transparent"
                                role="status"
                            >
                                <span className="sr-only">Loading...</span>
                            </div>
                        </div>
                    ) : fetchStatus.status === 'error' ? (
                        <p className="text-red-500">{fetchStatus.message}</p>
                    ) : (
                        <div className="overflow-x-auto">
                            <table className="w-full text-left">
                                <thead className="bg-gray-200 dark:bg-gray-700">
                                    <tr>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                            Server
                                        </th>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                            Status
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {serverStatus.map((status, index) => (
                                        <tr key={index} className="hover:bg-gray-100 dark:hover:bg-gray-900">
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                {status.server}
                                            </td>
                                            <td
                                                className={`px-4 py-2 border-b font-semibold ${getStatusStyle(
                                                    status.status
                                                )}`}
                                            >
                                                {status.status}
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Status;
