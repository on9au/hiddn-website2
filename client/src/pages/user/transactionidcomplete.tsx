// TransactionID.tsx

import React, { useEffect, useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import axios from 'axios';
import { PlanPayload, UserTransactionPayload, UserTransactionStatusEnum } from '../../bindings';
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
        <TransactionForm transaction={transaction} />
    );
};

interface TransactionFormProps {
    transaction: UserTransactionPayload;
}

const TransactionForm: React.FC<TransactionFormProps> = ({ transaction }) => {
    const navigate = useNavigate();

    const [planName, setPlanName] = useState<string>('');

    useEffect(() => {
        const fetchPlanName = async () => {
            const name = await getPlanName(transaction.plan_id);
            setPlanName(name);
        };
        fetchPlanName();
    }, [transaction.plan_id]);

    // Helper function to format date
    const formatDate = (dateStr: number) => {
        const date = new Date(dateStr * 1000);
        return date.toLocaleString();
    };

    // Helper function to display status
    const renderStatus = (status: UserTransactionStatusEnum) => {
        switch (status) {
            case UserTransactionStatusEnum.RequiresPaymentMethod:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Unpaid</span>;
            case UserTransactionStatusEnum.Processing:
                return <span className="px-2 py-1 text-sm text-blue-700 bg-blue-100 rounded dark:text-blue-300 dark:bg-blue-900">Processing</span>;
            case UserTransactionStatusEnum.Succeeded:
                return <span className="px-2 py-1 text-sm text-green-700 bg-green-100 rounded dark:text-green-300 dark:bg-green-900">Completed</span>;
            case UserTransactionStatusEnum.RequiresAction:
                return <span className="px-2 py-1 text-sm text-red-700 bg-red-100 rounded dark:text-red-300 dark:bg-red-900">Action Required</span>;
            case UserTransactionStatusEnum.RequiresConfirmation:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Confirmation Required</span>;
            case UserTransactionStatusEnum.RequiresCapture:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Capture Required</span>;
            case UserTransactionStatusEnum.Canceled:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Canceled</span>;
            case UserTransactionStatusEnum.Refunded:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Refunded</span>;
            default:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Unknown</span>;
        }
    };

    // Helper function to get plan name
    const getPlanName = async (planId: number) => {
        try {
            const response = await axios.get<PlanPayload>(`/api/plans/${planId}`, { withCredentials: true });
            return response.data.name;
        } catch (err) {
            console.error('Failed to get plan name:', err);
            return 'Failed to get plan name';
        }
    }

    return (
        <div className="flex flex-col items-center justify-center min-h-screen px-4">
            <div className="w-full max-w-2xl p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                <h2 className="mb-4 text-3xl font-semibold text-gray-800 dark:text-gray-200">
                    Transaction Details
                </h2>
                <div className="mb-6">
                    <p className="text-gray-700 dark:text-gray-300">
                        <strong>Transaction ID:</strong> {transaction.id}
                    </p>
                    <Link className="text-gray-700 dark:text-gray-300" to={"/user/plan/" + transaction.plan_id}>
                        <strong>Plan:</strong> {planName}
                    </Link>
                    <p className="text-gray-700 dark:text-gray-300">
                        <strong>Date:</strong> {formatDate(transaction.created_at)}
                    </p>
                    <p className="text-gray-700 dark:text-gray-300">
                        <strong>Status:</strong> {renderStatus(transaction.status)}
                    </p>
                </div>

                <div className="mb-6">
                    <hr className="border-gray-300 dark:border-gray-600" />
                </div>

                <div className="mb-6">
                    <p className="text-2xl font-bold text-gray-800 dark:text-gray-200">
                        Total: ${transaction.amount}
                    </p>
                </div>

                {/* Conditionally render payment form based on transaction status */}

                <div className="mb-6">
                    <hr className="border-gray-300 dark:border-gray-600" />
                </div>

                {/* Display success message if payment was successful */}
                <div className="flex flex-col items-center justify-center">
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
