import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';
import { AdminUser } from '../../bindings';

const AdminUserManagement: React.FC = () => {
    const [users, setUsers] = useState<AdminUser[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        const fetchUsers = async () => {
            try {
                const response = await axios.get('/api/admin/users', { withCredentials: true });
                setUsers(response.data);
                setLoading(false);
            } catch (err) {
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 401) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        }
                    }
                }
                setError('Failed to load users.');
                setLoading(false);
            }
        };

        fetchUsers();
    }, [navigate]);

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">User Management</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {loading ? (
                        <p>Loading...</p>
                    ) : error ? (
                        <p className="text-red-500">{error}</p>
                    ) : (
                        <div className="overflow-x-auto">
                            <table className="w-full text-left">
                                <thead>
                                    <tr>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">ID</th>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">Email</th>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">Admin</th>
                                        <th className="px-4 py-2 font-semibold text-gray-700 border-b dark:text-gray-300">Actions</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {users.map((user: AdminUser) => (
                                        <tr key={user.id} className="hover:bg-gray-100 dark:hover:bg-gray-900">
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">{user.id}</td>
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">{user.email}</td>
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                <input type="checkbox" checked={user.admin} readOnly />
                                            </td>
                                            <td className="px-4 py-2 text-gray-700 border-b dark:text-gray-300">
                                                <button className="px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700">Edit</button>
                                                <button className="px-4 py-2 ml-2 text-white bg-red-500 rounded-md hover:bg-red-600">Delete</button>
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default AdminUserManagement;