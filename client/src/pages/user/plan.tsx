import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { PlanPayload } from '../../bindings';
import { useNavigate } from 'react-router-dom';

const SkeletonPlan: React.FC = () => {
    return (
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
            {[1, 2, 3].map((_, index) => (
                <div key={index} className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800 animate-pulse">
                    <div className="w-1/2 h-8 mb-4 bg-gray-300 rounded"></div>
                    <div className="w-1/4 h-6 mb-2 bg-gray-300 rounded"></div>
                    <div className="w-1/3 h-6 mb-2 bg-gray-300 rounded"></div>
                    <div className="w-1/4 h-6 mb-2 bg-gray-300 rounded"></div>
                    <div className="w-full h-6 mb-4 bg-gray-300 rounded"></div>
                    <div className="w-full h-10 bg-gray-300 rounded"></div>
                </div>
            ))}
        </div>
    );
};

const Plan: React.FC = () => {
    const [plans, setPlans] = useState<PlanPayload[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Plans - HiddN';

        const fetchPlans = async () => {
            try {
                const response = await axios.get<PlanPayload[]>('/api/plans', {
                    withCredentials: true,
                });
                setPlans(response.data);
                setError(null);
            } catch (err) {
                console.error('Failed to load plans. Error:', err);
                setError('Failed to load plans.');
            } finally {
                setLoading(false);
            }
        };

        fetchPlans();
    }, []);

    const handlePurchase = (planId: number) => {
        navigate(`/user/plan/${planId}`);
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Available Plans</h1>
            </span>
            <div className="container mx-auto">
                {loading ? (
                    <SkeletonPlan />
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : plans.length > 0 ? (
                    <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                        {plans.map((plan) => (
                            <div key={plan.id} className="p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                                <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                    {plan.name}
                                </h2>
                                <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                    <strong>Price:</strong> ${plan.price.toFixed(2)}
                                </p>
                                {plan.data_limit !== null && (
                                    <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                        <strong>Data Limit:</strong> {plan.data_limit} GB
                                    </p>
                                )}
                                <p className="mb-2 text-base text-gray-700 dark:text-gray-300">
                                    <strong>Duration:</strong> {plan.duration_days} days
                                </p>
                                {plan.description && (
                                    <p className="mb-4 text-base text-gray-700 dark:text-gray-300">
                                        {plan.description}
                                    </p>
                                )}
                                <button
                                    className="w-full px-4 py-2 mt-4 text-white rounded-md bg-hiddn-500 hover:bg-hiddn-600 focus:outline-none"
                                    onClick={() => handlePurchase(plan.id)}
                                >
                                    Purchase
                                </button>
                            </div>
                        ))}
                    </div>
                ) : (
                    <p className="text-gray-700 dark:text-gray-300">No plans available at this time.</p>
                )}
            </div>
        </div>
    );
};

export default Plan;