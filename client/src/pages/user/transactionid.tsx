// TransactionID.tsx

import React, { useEffect, useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import axios from 'axios';
import { PlanPayload, UserTransactionPayload, UserTransactionStatusEnum } from '../../bindings';
import { loadStripe } from '@stripe/stripe-js';
import {
    Elements,
    CardElement,
    useStripe,
    useElements,
} from '@stripe/react-stripe-js';
// import { FaShoppingCart } from 'react-icons/fa';

const stripePromise = loadStripe('pk_test_51OaHvaHUfFNGnc8iKFFnkMOlcEjBFbnWz1ceTfBNK4lCwzLlHqOmXczBNP5mf0hVd69EtOgeUgcdXyUUSajKWOD400ek5bnGEe'); // Load Stripe public key from environment variables

const TransactionID: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const [transaction, setTransaction] = useState<UserTransactionPayload | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    // const navigate = useNavigate();

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
        <Elements stripe={stripePromise}>
            <TransactionForm transaction={transaction} />
        </Elements>
    );
};

interface TransactionFormProps {
    transaction: UserTransactionPayload;
}

const TransactionForm: React.FC<TransactionFormProps> = ({ transaction }) => {
    const stripe = useStripe();
    const elements = useElements();
    const navigate = useNavigate();

    const [processing, setProcessing] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);
    const [success, setSuccess] = useState<boolean>(false);

    const [planName, setPlanName] = useState<string>('');

    useEffect(() => {
        const fetchPlanName = async () => {
            const name = await getPlanName(transaction.plan_id);
            setPlanName(name);
        };
        fetchPlanName();
    }, [transaction.plan_id]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!stripe || !elements) {
            return;
        }

        setProcessing(true);
        setError(null);

        const cardElement = elements.getElement(CardElement);
        if (!cardElement) {
            setError('Card details not found.');
            setProcessing(false);
            return;
        }

        // Confirm the Payment Intent
        const { error: stripeError, paymentIntent } = await stripe.confirmCardPayment(
            transaction.stripe_payment_intent_id!, // Assuming this is the client secret
            {
                payment_method: {
                    card: cardElement,
                },
            }
        );

        if (stripeError) {
            setError(stripeError.message || 'Payment failed.');
            setProcessing(false);
            return;
        }

        if (paymentIntent && paymentIntent.status === 'succeeded') {
            // Notify the backend to update transaction status
            try {
                await axios.post(`/api/transactions/${transaction.id}/complete`, {}, {
                    withCredentials: true,
                });
                setSuccess(true);
            } catch (err) {
                console.error('Failed to update transaction status:', err);
                setError('Payment succeeded, but failed to update transaction status.');
            }
        } else {
            setError('Payment was not successful.');
        }

        setProcessing(false);
    };

    // Helper function to format date
    const formatDate = (dateStr: number) => {
        const date = new Date(dateStr * 1000);
        return date.toLocaleString();
    };

    // Helper function to display status
    const renderStatus = (status: UserTransactionStatusEnum) => {
        switch (status) {
            case UserTransactionStatusEnum.RequiresPaymentMethod:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded">Unpaid</span>;
            case UserTransactionStatusEnum.Processing:
                return <span className="px-2 py-1 text-sm text-blue-700 bg-blue-100 rounded">Processing</span>;
            case UserTransactionStatusEnum.Succeeded:
                return <span className="px-2 py-1 text-sm text-green-700 bg-green-100 rounded">Completed</span>;
            case UserTransactionStatusEnum.RequiresAction:
                return <span className="px-2 py-1 text-sm text-red-700 bg-red-100 rounded">Action Required</span>;
            case UserTransactionStatusEnum.RequiresConfirmation:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded">Confirmation Required</span>;
            case UserTransactionStatusEnum.RequiresCapture:
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded">Capture Required</span>;
            case UserTransactionStatusEnum.Canceled:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded">Canceled</span>;
            case UserTransactionStatusEnum.Refunded:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded">Refunded</span>;
            default:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded">Unknown</span>;
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
                    <strong>Plan:</strong> {planName}
                    <p className="text-gray-700 dark:text-gray-300">
                        <strong>Date:</strong> {formatDate(transaction.created_at)}
                    </p>
                    <p className="text-gray-700 dark:text-gray-300">
                        <strong>Status:</strong> {renderStatus(transaction.status)}
                    </p>
                    <Link to={"/user/plan/" + transaction.plan_id}>
                        <strong>Plan:</strong> {planName}
                    </Link>
                </div>

                {/* Conditionally render payment form based on transaction status */}

                <div className="mb-6">
                    <h3 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                        Complete Your Payment
                    </h3>
                    <p className="mb-4 text-gray-700 dark:text-gray-300">
                        Please enter your card details to complete the payment for your transaction.
                    </p>
                    {error && <p className="mb-4 text-red-500">{error}</p>}
                    <form onSubmit={handleSubmit}>
                        <div className="mb-6">
                            <label htmlFor="card-element" className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                                Card Details
                            </label>
                            <div className="p-3 border rounded-md bg-gray-50 dark:bg-gray-700">
                                <CardElement
                                    id="card-element"
                                    options={{
                                        style: {
                                            base: {
                                                fontSize: '16px',
                                                color: '#32325d',
                                                '::placeholder': {
                                                    color: '#a0aec0',
                                                },
                                            },
                                            invalid: {
                                                color: '#e53e3e',
                                            },
                                        },
                                    }}
                                />
                            </div>
                        </div>
                        <button
                            type="submit"
                            className={`w-full px-4 py-2 text-white bg-hiddn-500 rounded-md hover:bg-hiddn-600 focus:outline-none ${processing ? 'opacity-50 cursor-not-allowed' : ''
                                }`}
                            disabled={!stripe || processing}
                        >
                            {processing ? 'Processing...' : 'Pay Now'}
                        </button>
                    </form>
                </div>

                {/* Display success message if payment was successful */}
                {success && (
                    <div className="flex flex-col items-center justify-center min-h-screen">
                        <h1 className="mb-4 text-4xl font-semibold text-green-500">Payment Successful!</h1>
                        <p className="mb-6 text-xl text-gray-700 dark:text-gray-300">
                            Thank you for your purchase. Your VPN service is now active.
                        </p>
                        <button
                            className="px-6 py-3 text-white rounded-md bg-hiddn-500 hover:bg-hiddn-600 focus:outline-none"
                            onClick={() => navigate("/user/dashboard")}
                        >
                            Go to Dashboard
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}

export default TransactionID;
