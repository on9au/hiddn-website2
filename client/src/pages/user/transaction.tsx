import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { UserTransactionPayload, UserTransactionStatusEnum } from '../../bindings';
import { useNavigate } from 'react-router-dom';

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

const Transaction: React.FC = () => {
    const [transactions, setTransactions] = useState<UserTransactionPayload[]>([]);
    const [fetchStatus, setFetchStatus] = useState<FetchTransactionsEnum>({ status: 'loading' });
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Transactions - HiddN';

        const fetchTransactions = async () => {
            try {
                const response = await axios.get<UserTransactionPayload[]>('/api/transactions', {
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
    const getStatusStyle = (status: UserTransactionStatusEnum) => {
        switch (status) {
            case UserTransactionStatusEnum.RequiresPaymentMethod:
                return <td className="px-4 py-2 font-semibold text-yellow-700 border-b">Unpaid</td>;
            case UserTransactionStatusEnum.Processing:
                return <td className="px-4 py-2 font-semibold text-blue-700">Processing</td>;
            case UserTransactionStatusEnum.Succeeded:
                return <td className="px-4 py-2 font-semibold text-green-700">Completed</td>;
            case UserTransactionStatusEnum.RequiresAction:
                return <td className="px-4 py-2 font-semibold text-red-700">Action Required</td>;
            case UserTransactionStatusEnum.RequiresConfirmation:
                return <td className="px-4 py-2 font-semibold text-yellow-700">Confirmation Required</td>;
            case UserTransactionStatusEnum.RequiresCapture:
                return <td className="px-4 py-2 font-semibold text-yellow-700">Capture Required</td>;
            case UserTransactionStatusEnum.Canceled:
                return <td className="px-4 py-2 font-semibold text-gray-700">Canceled</td>;
            case UserTransactionStatusEnum.Refunded:
                return <td className="px-4 py-2 font-semibold text-gray-700">Refunded</td>;
            default:
                return <td className="px-4 py-2 font-semibold text-gray-700">Unknown</td>;
        }
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
                                            Status
                                        </th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {transactions.map((transaction) => (
                                        <tr
                                            key={transaction.id}
                                            className="cursor-pointer hover:bg-gray-100 dark:hover:bg-gray-900"
                                            onClick={() => navigate(`/user/transaction/${transaction.id}`)}
                                        >
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                {transaction.id}
                                            </td>
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                ${transaction.amount.toFixed(2)}
                                            </td>
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                {new Date(transaction.created_at * 1000).toLocaleString(undefined, { timeZoneName: 'short' })}
                                            </td>
                                            {getStatusStyle(transaction.status)}
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    ) : (
                        <p className="text-gray-700 dark:text-gray-300">No transactions found.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Transaction;