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
        document.title = 'Hiddn | Server Status';

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

    if (fetchStatus.status === 'loading') {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <div
                    className="inline-block w-8 h-8 border-4 border-current border-solid rounded-full animate-spin border-r-transparent"
                    role="status"
                >
                    <span className="sr-only">Loading...</span>
                </div>
            </div>
        );
    }

    if (fetchStatus.status === 'error') {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <p className="text-red-500">{fetchStatus.message}</p>
            </div>
        );
    }

    return (
        <div className="flex flex-col items-center min-h-screen pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Server Status</h1>
            </span>
            <div className="container">
                <div className="overflow-x-auto">
                    <table className="min-w-full bg-white dark:bg-gray-800">
                        <thead className="bg-gray-200 dark:bg-gray-700">
                            <tr>
                                <th className="px-4 py-2 border-b">Server</th>
                                <th className="px-4 py-2 border-b">Status</th>
                            </tr>
                        </thead>
                        <tbody>
                            {serverStatus.map((status, index) => (
                                <tr key={index} className="hover:bg-gray-100 dark:hover:bg-gray-900">
                                    <td className="px-4 py-2 border-b">{status.server}</td>
                                    <td className="px-4 py-2 border-b">{status.status}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
};

export default Status;
