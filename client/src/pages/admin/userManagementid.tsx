import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate, useParams } from 'react-router-dom';
import { AdminUser } from '../../bindings/AdminUser';
import { AdminUserModify } from '../../bindings/AdminUserModify';

const UserManagementId: React.FC = () => {
    const { id } = useParams<{ id: string }>();
    const [user, setUser] = useState<AdminUser | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [email, setEmail] = useState('');
    const [marzbanUsername, setMarzbanUsername] = useState('');
    const [isAdmin, setIsAdmin] = useState(false);
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'User Management - HiddN';
        const fetchUser = async () => {
            try {
                const response = await axios.get(`/api/admin/users/${id}`, { withCredentials: true });
                setUser(response.data);
                setEmail(response.data.email);
                setMarzbanUsername(response.data.marzban_username || '');
                setIsAdmin(response.data.admin);
                setLoading(false);
            } catch (err) {
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 401 || err.response.status === 403) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        }
                    }
                }
                setError('Failed to load user.');
                setLoading(false);
            }
        };

        fetchUser();
    }, [id, navigate]);

    const handleUpdateUser = async () => {
        const payload: AdminUserModify = {
            email: email,
            marzban_username: marzbanUsername.trim() === '' ? null : marzbanUsername,
            admin: isAdmin,
        }

        try {
            await axios.put(
                `/api/admin/users/${id}`,
                payload,
                { withCredentials: true }
            );
            alert('User updated successfully.');
            navigate('/admin/user-management');
        } catch (err) {
            if (axios.isAxiosError(err) && err.response) {
                setError(err.response.data.message || 'Failed to update user. Please try again.');
            } else {
                setError('An unexpected error occurred. Please try again.');
            }
        }
    };

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
                    ) : user ? (
                        <div>
                            <div className="mb-4">
                                <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                                    Email
                                </label>
                                <input
                                    type="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    className="w-full p-2 mb-4 border rounded dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600"
                                />
                            </div>
                            <div className="mb-4">
                                <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                                    Marzban Username
                                </label>
                                <input
                                    type="text"
                                    value={marzbanUsername}
                                    onChange={(e) => setMarzbanUsername(e.target.value)}
                                    className="w-full p-2 mb-4 border rounded dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600"
                                />
                            </div>
                            <div className="mb-4">
                                <label className="block mb-2 text-sm font-medium text-gray-700 dark:text-gray-300">
                                    Admin
                                </label>
                                <input
                                    type="checkbox"
                                    checked={isAdmin}
                                    onChange={(e) => setIsAdmin(e.target.checked)}
                                    className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                                />
                            </div>
                            <button
                                onClick={handleUpdateUser}
                                className="px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none"
                            >
                                Update User
                            </button>
                        </div>
                    ) : (
                        <p className="text-gray-700 dark:text-gray-300">No user data available.</p>
                    )}
                </div>
            </div>
        </div>
    );
};

export default UserManagementId;