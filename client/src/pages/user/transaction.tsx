import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { UserTransactionPayload } from '../../bindings';
import { useNavigate } from 'react-router-dom';

type FetchTransactionsEnum =
    | { status: 'loading' }
    | { status: 'success' }
    | { status: 'error'; message: string };

const Transaction: React.FC = () => {
    const [transactions, setTransactions] = useState<UserTransactionPayload[]>([]);
    const [fetchStatus, setFetchStatus] = useState<FetchTransactionsEnum>({ status: 'loading' });
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Hiddn | Transactions';

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

        fetchTransactions();
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
                <h1 className="text-4xl font-semibold">Transactions</h1>
            </span>
            <div className="container">
                <div className="overflow-x-auto">
                    <table className="min-w-full bg-white dark:bg-gray-800">
                        <thead className="bg-gray-200 dark:bg-gray-700">
                            <tr>
                                <th className="px-4 py-2 border-b">Transaction ID</th>
                                <th className="px-4 py-2 border-b">Amount</th>
                                <th className="px-4 py-2 border-b">Date</th>
                                <th className="px-4 py-2 border-b">Payment Method</th>
                                <th className="px-4 py-2 border-b">Status</th>
                            </tr>
                        </thead>
                        <tbody>
                            {transactions.map((transaction) => (
                                <tr key={transaction.transaction_id}>
                                    <td className="px-4 py-2 border-b">{transaction.transaction_id}</td>
                                    <td className="px-4 py-2 border-b">
                                        ${transaction.amount.toFixed(2)}
                                    </td>
                                    <td className="px-4 py-2 border-b">
                                        {new Date(transaction.transaction_date).toLocaleDateString()}
                                    </td>
                                    <td className="px-4 py-2 border-b">
                                        {transaction.payment_method || 'N/A'}
                                    </td>
                                    <td className="px-4 py-2 border-b">{transaction.status}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    );
};

export default Transaction;
