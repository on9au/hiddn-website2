// TransactionID.tsx

import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import axios from 'axios';
import { UserTransactionPayload } from '../../bindings';
// import { FaShoppingCart } from 'react-icons/fa';

const TransactionIDComplete: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const [transaction, setTransaction] = useState<UserTransactionPayload | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchTransaction = async () => {
            try {
                const response = await axios.get<UserTransactionPayload>(`/api/transaction/${id}`, {
                    withCredentials: true,
                });
                setTransaction(response.data);
                setError(null);
            } catch (err) {
                console.error('Failed to load transaction. Error:', err);
                setError('Failed to load transaction.');
                setTransaction(null);
            } finally {
                setLoading(false);
            }
        };

        document.title = `Transaction ${id} - HiddN`;
        fetchTransaction();
    }, [id]);

    if (loading) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen">
                <div
                    className="inline-block w-12 h-12 border-4 border-current border-blue-500 border-solid rounded-full animate-spin border-r-transparent"
                    role="status"
                >
                    <span className="sr-only">Loading...</span>
                </div>
            </div>
        );
    }

    if (error || !transaction) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen">
                <p className="text-red-500">{error || 'Transaction not found.'}</p>
            </div>
        );
    }

    return (
        <TransactionForm />
    );
};

const TransactionForm: React.FC = () => {
    const navigate = useNavigate();

    return (
        <div className="flex flex-col items-center justify-center min-h-screen px-4">
            <div className="w-full max-w-2xl p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                <div className="flex flex-col items-center justify-center min-h-screen">
                    <h1 className="mb-4 text-4xl font-semibold text-green-500">Payment Successful!</h1>
                    <p className="mb-6 text-xl text-gray-700 dark:text-gray-300">
                        Thank you for your purchase. The plan has been added to your account.
                    </p>
                    <button
                        className="px-6 py-3 text-white rounded-md bg-hiddn-500 hover:bg-hiddn-600 focus:outline-none"
                        onClick={() => navigate("/user/dashboard")}
                    >
                        Go to Dashboard
                    </button>
                </div>
            </div>
        </div>
    );
}

export default TransactionIDComplete;
