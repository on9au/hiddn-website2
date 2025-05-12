import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import { UserTransaction } from '../../bindings/UserTransaction';
import { UserTransactionStatus } from '../../bindings/UserTransactionStatus';

type FetchTransactionsEnum =
    | { status: 'loading' }
    | { status: 'success' }
    | { status: 'error'; message: string };

const SkeletonTransaction: React.FC = () => {
    return (
        <div className="overflow-x-auto">
            <table className="w-full text-left">
                <thead className="bg-gray-200 dark:bg-gray-700">
                    <tr>
                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                            Transaction ID
                        </th>
                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                            Amount
                        </th>
                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                            Date
                        </th>
                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                            Payment Method
                        </th>
                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                            Status
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {[1, 2, 3].map((_, index) => (
                        <tr key={index} className="hover:bg-gray-100 dark:hover:bg-gray-900">
                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                <div className="w-24 h-6 bg-gray-300 rounded animate-pulse"></div>
                            </td>
                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                <div className="w-16 h-6 bg-gray-300 rounded animate-pulse"></div>
                            </td>
                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                <div className="w-20 h-6 bg-gray-300 rounded animate-pulse"></div>
                            </td>
                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                <div className="w-24 h-6 bg-gray-300 rounded animate-pulse"></div>
                            </td>
                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                <div className="w-16 h-6 bg-gray-300 rounded animate-pulse"></div>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

const AdminTransaction: React.FC = () => {
    const [transactions, setTransactions] = useState<UserTransaction[]>([]);
    const [fetchStatus, setFetchStatus] = useState<FetchTransactionsEnum>({ status: 'loading' });
    const [userIdFilter, setUserIdFilter] = useState<string>('');
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Transactions - HiddN';

        const fetchTransactions = async () => {
            try {
                const response = await axios.get<UserTransaction[]>('/api/admin/transactions', {
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    withCredentials: true, // Include cookies for authentication if needed
                });

                setTransactions(response.data);
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
                            message: 'Failed to fetch transactions. Please try again later. Error: ' + error.message,
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

        fetchTransactions();
    }, [navigate]);

    // Function to get status styles
    const getStatusStyle = (status: UserTransactionStatus) => {
        switch (status) {
            case "RequiresPaymentMethod":
                return <td className="px-4 py-2 font-semibold text-yellow-700 border-b">Unpaid</td>;
            case "Processing":
                return <td className="px-4 py-2 font-semibold text-blue-700 border-b">Processing</td>;
            case "Succeeded":
                return <td className="px-4 py-2 font-semibold text-green-700 border-b">Completed</td>;
            case "RequiresAction":
                return <td className="px-4 py-2 font-semibold text-red-700 border-b">Action Required</td>;
            case "RequiresConfirmation":
                return <td className="px-4 py-2 font-semibold text-yellow-700 border-b">Confirmation Required</td>;
            case "RequiresCapture":
                return <td className="px-4 py-2 font-semibold text-yellow-700 border-b">Capture Required</td>;
            case "Cancelled":
                return <td className="px-4 py-2 font-semibold text-gray-700 border-b">Cancelled</td>;
            case "Refunded":
                return <td className="px-4 py-2 font-semibold text-gray-700 border-b">Refunded</td>;
            default:
                return <td className="px-4 py-2 font-semibold text-gray-700 border-b">Unknown</td>;
        }
    };

    // Function to filter transactions by user ID
    const filteredTransactions = transactions.filter(transaction =>
        userIdFilter === '' ||
        (transaction.user_id && transaction.user_id.toString().includes(userIdFilter))
    );

    const clearFilter = () => {
        setUserIdFilter('');
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Transactions</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {fetchStatus.status === 'loading' ? (
                        <SkeletonTransaction />
                    ) : fetchStatus.status === 'error' ? (
                        <p className="text-red-500">{fetchStatus.message}</p>
                    ) : transactions.length > 0 ? (
                        <div>
                            <div className="flex items-end mb-4">
                                <div className="flex-1">
                                    <label htmlFor="userIdFilter" className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                                        Filter by User ID
                                    </label>
                                    <div className="flex">
                                        <input
                                            type="text"
                                            id="userIdFilter"
                                            value={userIdFilter}
                                            onChange={(e) => setUserIdFilter(e.target.value)}
                                            placeholder="Enter User ID"
                                            className="w-full px-4 py-2 border border-gray-300 rounded-l-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                                        />
                                        {userIdFilter && (
                                            <button
                                                onClick={clearFilter}
                                                className="px-4 py-2 bg-gray-200 rounded-r-lg hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500 dark:text-white"
                                            >
                                                Clear
                                            </button>
                                        )}
                                    </div>
                                </div>
                            </div>

                            {filteredTransactions.length === 0 ? (
                                <p className="mt-4 text-gray-700 dark:text-gray-300">No transactions match the filter.</p>
                            ) : (
                                <div className="overflow-x-auto">
                                    <table className="w-full text-left">
                                        <thead className="bg-gray-200 dark:bg-gray-700">
                                            <tr>
                                                <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                                    Transaction ID
                                                </th>
                                                <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                                    User ID
                                                </th>
                                                <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                                    Amount
                                                </th>
                                                <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                                    Date
                                                </th>
                                                <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">
                                                    Status
                                                </th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {filteredTransactions.map((transaction) => (
                                                <tr
                                                    key={transaction.id}
                                                    className="cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-900"
                                                    onClick={() => navigate(`/admin/transaction/${transaction.id}`)}
                                                >
                                                    <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                        {transaction.id}
                                                    </td>
                                                    <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                        {transaction.user_id}
                                                    </td>
                                                    <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                        ${Number(transaction.amount).toFixed(2)}
                                                    </td>
                                                    <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                        {new Date(transaction.created_at).toLocaleString(undefined, { timeZoneName: 'short' })}
                                                    </td>
                                                    {getStatusStyle(transaction.status)}
                                                </tr>
                                            ))}
                                        </tbody>
                                    </table>
                                </div>
                            )}
                        </div>
                    ) : (
                        <p className="text-gray-700 dark:text-gray-300">No transactions found.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default AdminTransaction;