// TransactionID.tsx

import React, { useEffect, useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import axios from 'axios';
import { loadStripe } from '@stripe/stripe-js';
import {
    Elements,
    useStripe,
    useElements,
    PaymentElement,
} from '@stripe/react-stripe-js';
import { UserTransaction } from '../../bindings/UserTransaction';
import { UserTransactionStatus } from '../../bindings/UserTransactionStatus';
import { Plan } from '../../bindings/Plan';
// import { FaShoppingCart } from 'react-icons/fa';

const stripePromise = loadStripe('pk_test_51OaHvaHUfFNGnc8iKFFnkMOlcEjBFbnWz1ceTfBNK4lCwzLlHqOmXczBNP5mf0hVd69EtOgeUgcdXyUUSajKWOD400ek5bnGEe'); // Load Stripe public key from environment variables

const TransactionID: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const [transaction, setTransaction] = useState<UserTransaction | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const [clientSecret, setClientSecret] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        // Get the client secret for the transaction from session storage, then the backend
        const clientSecret = sessionStorage.getItem(`order:${id}`);
        if (clientSecret) {
            setClientSecret(clientSecret);
        } else {
            const fetchClientSecret = async () => {
                try {
                    const response = await axios.get<string>(`/api/transactions/${id}/secret`, {
                        withCredentials: true,
                    });
                    setClientSecret(response.data);
                    sessionStorage.setItem(`order:${id}`, response.data);
                    setError(null);
                } catch (err) {
                    console.error('Failed to get client secret:', err);
                    setError('Failed to get client secret.');
                    setClientSecret(null);
                }
            };
            fetchClientSecret();
        }
    }, [id])

    useEffect(() => {
        const fetchTransaction = async () => {
            try {
                const response = await axios.get<UserTransaction>(`/api/transactions/${id}`, {
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


    const handleCancelTransaction = async () => {
        if (!transaction) return;

        const confirmCancel = window.confirm("Are you sure you want to cancel this transaction?");
        if (!confirmCancel) return;

        try {
            await axios.post(`/api/transactions/${transaction.id}/cancel`, {}, {
                withCredentials: true,
            });
            alert("Transaction canceled successfully.");
            navigate('/user/transaction');
        } catch (err) {
            console.error('Failed to cancel transaction:', err);
            alert('Failed to cancel transaction.');
        }
    };

    if (error || !transaction) {
        return (
            <div className="flex flex-col items-center justify-center min-h-screen">
                <p className="text-red-500">{error || 'Transaction not found.'}</p>
            </div>
        );
    }

    return (
        <Elements
            stripe={stripePromise}
            options={{
                clientSecret: clientSecret || undefined,
                appearance: {
                    theme: document.documentElement.classList.contains('dark') ? 'night' : 'stripe'
                },
                locale: 'auto',
            }}
        >
            <TransactionForm transaction={transaction} clientSecret={clientSecret} onCancel={handleCancelTransaction} />
        </Elements>
    );
};

interface TransactionFormProps {
    transaction: UserTransaction;
    clientSecret: string | null;
    onCancel: () => void;
}

const TransactionForm: React.FC<TransactionFormProps> = ({ transaction, onCancel }) => {
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

        // const cardElement = elements.getElement(CardElement);
        // if (!cardElement) {
        //     setError('Card details not found.');
        //     setProcessing(false);
        //     return;
        // }

        // Confirm the Payment Intent
        const result = await stripe.confirmPayment({
            //`Elements` instance that was used to create the Payment Element
            elements,
            confirmParams: {
                return_url: `${window.location.origin}/user/transaction/${transaction.id}/complete`,
            },
        });


        if (result.error) {
            // Show error to your customer (for example, payment details incomplete)
            console.log(result.error.message);
            setError(result.error.message || 'An unknown error occurred.');
        } else {
            // Your customer will be redirected to your `return_url`. For some payment
            // methods like iDEAL, your customer will be redirected to an intermediate
            // site first to authorize the payment, then redirected to the `return_url`.
            setSuccess(true);
        }


        setProcessing(false);
    };

    // Helper function to format date
    const formatDate = (dateStr: string) => {
        const date = new Date(dateStr);
        return date.toLocaleString();
    };

    // Helper function to display status
    const renderStatus = (status: UserTransactionStatus) => {
        switch (status) {
            case "RequiresPaymentMethod":
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Unpaid</span>;
            case "Processing":
                return <span className="px-2 py-1 text-sm text-blue-700 bg-blue-100 rounded dark:text-blue-300 dark:bg-blue-900">Processing</span>;
            case "Succeeded":
                return <span className="px-2 py-1 text-sm text-green-700 bg-green-100 rounded dark:text-green-300 dark:bg-green-900">Completed</span>;
            case "RequiresAction":
                return <span className="px-2 py-1 text-sm text-red-700 bg-red-100 rounded dark:text-red-300 dark:bg-red-900">Action Required</span>;
            case "RequiresConfirmation":
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Confirmation Required</span>;
            case "RequiresCapture":
                return <span className="px-2 py-1 text-sm text-yellow-700 bg-yellow-100 rounded dark:text-yellow-300 dark:bg-yellow-900">Capture Required</span>;
            case "Canceled":
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Canceled</span>;
            case "Refunded":
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Refunded</span>;
            default:
                return <span className="px-2 py-1 text-sm text-gray-700 bg-gray-100 rounded dark:text-gray-300 dark:bg-gray-900">Unknown</span>;
        }
    };

    // Helper function to get plan name
    const getPlanName = async (planId: number) => {
        try {
            const response = await axios.get<Plan>(`/api/plans/${planId}`, { withCredentials: true });
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
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                        All prices are in AUD and include GST.
                    </p>
                </div>

                {/* Conditionally render payment form based on transaction status */}

                {transaction.status === "RequiresPaymentMethod" && !success && (
                    <>
                        <div className="mb-6">
                            <hr className="border-gray-300 dark:border-gray-600" />
                        </div>
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
                                    <div className="p-3 text-gray-900 border rounded-md bg-gray-50 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-300">
                                        <PaymentElement
                                        // id="card-element"
                                        // options={
                                        //     {}
                                        // }
                                        // stripe={stripePromise}
                                        // // options={{
                                        // //     style: {
                                        // //         base: {
                                        // //             fontSize: '16px',
                                        // //             color: '#32325d',
                                        // //             '::placeholder': {
                                        // //                 color: '#a0aec0',
                                        // //             },
                                        // //             backgroundColor: '#f7fafc',
                                        // //             ':-webkit-autofill': {
                                        // //                 color: '#f7fafc',
                                        // //             },
                                        // //         },
                                        // //         invalid: {
                                        // //             color: '#e53e3e',
                                        // //         },
                                        // //         complete: {
                                        // //             color: '#38a169',
                                        // //         },
                                        // //     },
                                        // // }}
                                        // className="dark:bg-gray-700 dark:text-gray-300"
                                        />
                                    </div>
                                    <span className="block mt-2 text-xs text-gray-500 dark:text-gray-400">
                                        All transactions are secured and encrypted by Stripe.
                                    </span>
                                    <span className="block mt-1 text-xs text-gray-500 dark:text-gray-400">
                                        We do not store your card details.
                                    </span>
                                </div>
                                <button
                                    type="submit"
                                    className={`w-full px-4 py-2 text-white bg-hiddn-500 rounded-md hover:bg-hiddn-600 focus:outline-none ${processing ? 'opacity-50 cursor-not-allowed' : ''}`}
                                    disabled={!stripe || processing}
                                >
                                    {processing ? 'Processing...' : 'Pay Now'}
                                </button>
                            </form>
                        </div>
                    </>
                )}

                {/* Display success message if payment was successful */}
                {success && (
                    <>
                        <div className="mb-6">
                            <hr className="border-gray-300 dark:border-gray-600" />
                        </div>
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
                    </>
                )}

                {/* Cancel Transaction Button */}
                {transaction.status === "RequiresPaymentMethod" && !success && (
                    <div className="mt-6">
                        <button
                            className="px-4 py-2 text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none"
                            onClick={onCancel}
                        >
                            Cancel Transaction
                        </button>
                    </div>
                )}
            </div>
        </div>
    );
}

export default TransactionID;
