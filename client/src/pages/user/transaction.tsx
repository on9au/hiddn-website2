import React, { useEffect, useState } from 'react';
import axios from 'axios';

interface UserTransactionPayload {
    transaction_id: number;
    amount: number;
    transaction_date: string;
    payment_method?: string;
    status: string;
}

const Transaction: React.FC = () => {
    const [transactions, setTransactions] = useState<UserTransactionPayload[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        document.title = 'Hiddn | Transactions';

        const fetchTransactions = async () => {
            try {
                const response = await axios.get<UserTransactionPayload[]>('/api/transactions', {
                    withCredentials: true, // Include cookies for authentication if needed
                });
                setTransactions(response.data);
            } catch (err) {
                setError('Failed to fetch transactions.' + err);
            } finally {
                setLoading(false);
            }
        };

        fetchTransactions();
    }, []);

    if (loading) {
        return <div className="flex items-center justify-center min-h-screen">
            <div
                className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
                role="status">
                <span
                    className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                >Loading...</span>
            </div>
        </div>;
    }

    if (error) {
        return <div className="flex items-center justify-center min-h-screen">{error}</div>;
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
