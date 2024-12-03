import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { NewPlanPayload, PlanPayload } from '../../bindings';
// import { useNavigate } from 'react-router-dom';

const AdminPlanManager: React.FC = () => {
    const [plans, setPlans] = useState<PlanPayload[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);
    const [newPlan, setNewPlan] = useState<NewPlanPayload>({
        name: '',
        price: 0,
        data_limit: undefined,
        duration_days: 0,
        description: '',
    });
    // const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Admin Plan Manager - HiddN';

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

    const handleCreatePlan = async () => {
        try {
            const response = await axios.post('/api/admin/plans', newPlan, {
                withCredentials: true,
            });
            setPlans([...plans, response.data]);
            setNewPlan({
                name: '',
                price: 0,
                data_limit: undefined,
                duration_days: 0,
                description: '',
            });
        } catch (err) {
            console.error('Failed to create plan. Error:', err);
            setError('Failed to create plan.');
        }
    };

    const handleDeletePlan = async (planId: number) => {
        try {
            await axios.delete(`/api/admin/plans/${planId}`, {
                withCredentials: true,
            });
            setPlans(plans.filter(plan => plan.id !== planId));
        } catch (err) {
            console.error('Failed to delete plan. Error:', err);
            setError('Failed to delete plan.');
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Admin Plan Manager</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">Create New Plan</h2>
                    <input
                        type="text"
                        placeholder="Name"
                        value={newPlan.name}
                        onChange={(e) => setNewPlan({ ...newPlan, name: e.target.value })}
                        className="w-full p-2 mb-4 border rounded"
                    />
                    <input
                        type="number"
                        placeholder="Price"
                        value={newPlan.price}
                        onChange={(e) => setNewPlan({ ...newPlan, price: parseFloat(e.target.value) })}
                        className="w-full p-2 mb-4 border rounded"
                    />
                    <input
                        type="number"
                        placeholder="Data Limit (GB)"
                        value={newPlan.data_limit}
                        onChange={(e) => setNewPlan({ ...newPlan, data_limit: parseFloat(e.target.value) })}
                        className="w-full p-2 mb-4 border rounded"
                    />
                    <input
                        type="number"
                        placeholder="Duration (days)"
                        value={newPlan.duration_days}
                        onChange={(e) => setNewPlan({ ...newPlan, duration_days: parseInt(e.target.value) })}
                        className="w-full p-2 mb-4 border rounded"
                    />
                    <textarea
                        placeholder="Description"
                        value={newPlan.description}
                        onChange={(e) => setNewPlan({ ...newPlan, description: e.target.value })}
                        className="w-full p-2 mb-4 border rounded"
                    />
                    <button
                        onClick={handleCreatePlan}
                        className="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600"
                    >
                        Create Plan
                    </button>
                </div>
                <div className="w-full p-6 mt-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">Existing Plans</h2>
                    {loading ? (
                        <p>Loading...</p>
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
                                        className="w-full px-4 py-2 mt-4 text-white bg-red-500 rounded-md hover:bg-red-600 focus:outline-none"
                                        onClick={() => handleDeletePlan(plan.id)}
                                    >
                                        Delete
                                    </button>
                                </div>
                            ))}
                        </div>
                    ) : (
                        <p className="text-gray-700 dark:text-gray-300">No plans available at this time.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default AdminPlanManager;