import React from 'react';
import { ServerStatusPayload } from '../../bindings';
import { useNavigate } from 'react-router-dom';

type FetchServerStatusEnum =
    | { 'status': 'loading' }
    | { 'status': 'success' }
    | { 'status': 'error', 'message': string }

const Status: React.FC = () => {
    // Server status table. Server returns a Vec<ServerStatus>.
    const navigate = useNavigate();
    const [serverStatus, setServerStatus] = React.useState([] as ServerStatusPayload[]);
    const [fetchServerStatus, setFetchServerStatus] = React.useState({ 'status': 'loading' } as FetchServerStatusEnum);

    // Get server status.
    React.useEffect(() => {
        async function fetchData() {
            try {
                const result = await fetch(`/api/server_status`, {
                    method: 'GET',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                });
                if (result.ok) {
                    setFetchServerStatus({ 'status': 'success' });
                    const json = await result.json();
                    setServerStatus(json);
                } else if (result.status === 401) {
                    setFetchServerStatus({ 'status': 'error', message: 'Unauthorized. Please log in.' });
                    navigate('/logout');
                } else {
                    setFetchServerStatus({ 'status': 'error', message: result.statusText });
                }
            } catch (error) {
                setFetchServerStatus({ 'status': 'error', message: 'Failed to fetch server status. Try again later. Error: ' + error });
            }
        };
        fetchData();
    }, [navigate]);

    return (
        <div className="flex flex-col items-center min-h-screen pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Server Status</h1>
            </span>
            <div className="w-full p-4 mb-6 bg-gray-200 dark:bg-gray-800 rounded-2xl">
                {/* Server status table. Server returns a Vec<ServerStatus>. */}
                {fetchServerStatus.status === 'loading' && (
                    <div className="flex justify-center">
                        <div
                            className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
                            role="status">
                            <span
                                className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                            >Loading...</span>
                        </div>
                    </div>
                )}
                {fetchServerStatus.status === 'error' && (
                    <p className="text-red-500">{fetchServerStatus.message}</p>
                )}
                {fetchServerStatus.status === 'success' && (
                    <table className="w-full">
                        <thead>
                            <tr>
                                <th className="text-left">Server</th>
                                <th className="text-left">Status</th>
                            </tr>
                        </thead>
                        <tbody>
                            {serverStatus.map((status, index) => (
                                <tr key={index}>
                                    <td>{status.server}</td>
                                    <td>{status.status}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                )}
            </div>
        </div>
    );
};

export default Status;
