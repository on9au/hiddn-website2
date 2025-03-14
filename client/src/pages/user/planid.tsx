import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import axios from 'axios';
import { CreateOrderResponsePayload, PlanPayload } from '../../bindings';
import { FaShoppingCart } from 'react-icons/fa';

const SkeletonPlanID: React.FC = () => {
    return (
        <div className="flex flex-col items-center p-6 bg-white rounded-lg shadow-md dark:bg-gray-800 animate-pulse">
            <div className="w-1/2 h-8 mb-4 bg-gray-300 rounded"></div>
            <div className="w-1/4 h-6 mb-2 bg-gray-300 rounded"></div>
            <div className="w-1/3 h-6 mb-2 bg-gray-300 rounded"></div>
            <div className="w-1/4 h-6 mb-2 bg-gray-300 rounded"></div>
            <div className="w-full h-6 mb-4 bg-gray-300 rounded"></div>
            <div className="w-full h-10 bg-gray-300 rounded"></div>
        </div>
    );
};

const PlanID: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const navigate = useNavigate();

    const [plan, setPlan] = useState<PlanPayload | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const [isOrdering, setIsOrdering] = useState<boolean>(false);

    useEffect(() => {
        const fetchPlanDetails = async () => {
            try {
                const response = await axios.get<PlanPayload>(`/api/plans/${id}`, {
                    withCredentials: true,
                });
                setPlan(response.data);
                setError(null);
            } catch (err) {
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 404) {
                            setError('Plan not found.');
                        } else if (err.response.status === 401) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        } else {
                            setError(err.response.data.message || 'Failed to load plan details.');
                        }
                    } else {
                        setError('Network error. Please try again.');
                    }
                } else {
                    setError('An unexpected error occurred.');
                }
                setPlan(null);
            } finally {
                setLoading(false);
            }
        };

        document.title = `Plan ${id} - HiddN`;
        fetchPlanDetails();
    }, [id, navigate]);

    const handleOrder = async () => {
        if (!plan) return;

        setIsOrdering(true);
        try {
            const response = await axios.post<CreateOrderResponsePayload>('/api/orders', {
                plan_id: plan.id,
            }, {
                withCredentials: true,
            });

            const transaction = response.data;

            // Store the client secret in sessionStorage, indexed by the order ID
            sessionStorage.setItem("order:" + transaction.order_id.toString(), transaction.payment_intent_client_secret);

            navigate(`/user/transaction/${transaction.order_id}`);
        } catch (err) {
            console.error('Failed to create order:', err);
            alert('Failed to create order. Please try again.');
        } finally {
            setIsOrdering(false);
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Plan Details</h1>
            </span>
            <div className="container mx-auto">
                {loading ? (
                    <SkeletonPlanID />
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : plan ? (
                    <div className="flex flex-col items-center p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                        <h2 className="mb-4 text-3xl font-semibold text-gray-800 dark:text-gray-200">
                            {plan.name}
                        </h2>
                        <p className="mb-2 text-xl text-gray-700 dark:text-gray-300">
                            <strong>Price:</strong> ${plan.price.toFixed(2)}
                        </p>
                        {plan.data_limit !== null && (
                            <p className="mb-2 text-xl text-gray-700 dark:text-gray-300">
                                <strong>Data Limit:</strong> {plan.data_limit} GB
                            </p>
                        )}
                        <p className="mb-4 text-xl text-gray-700 dark:text-gray-300">
                            <strong>Duration:</strong> {plan.duration_days} days
                        </p>
                        {plan.description && (
                            <p className="mb-6 text-base text-center text-gray-700 dark:text-gray-300">
                                {plan.description}
                            </p>
                        )}
                        <button
                            className={`mb-4 flex items-center px-6 py-3 text-white bg-hiddn-500 rounded-md hover:bg-hiddn-600 focus:outline-none ${isOrdering ? 'opacity-50 cursor-not-allowed' : ''
                                }`}
                            onClick={handleOrder}
                            disabled={isOrdering}
                        >
                            <FaShoppingCart className="mr-2" />
                            {isOrdering ? 'Ordering...' : 'Purchase'}
                        </button>
                        <p className="mb-2 text-sm text-gray-400 dark:text-gray-600">
                            All prices are in AUD and include GST.
                        </p>
                    </div>
                ) : (
                    <p className="text-gray-700 dark:text-gray-300">No plan details available.</p>
                )}
            </div>
        </div>
    );
};

export default PlanID;