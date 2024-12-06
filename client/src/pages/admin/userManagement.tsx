import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate, Link } from 'react-router-dom';
import { AdminUser } from '../../bindings';

const AdminUserManagement: React.FC = () => {
    const [users, setUsers] = useState<AdminUser[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [myId, setMyId] = useState<number | null>(null);
    const navigate = useNavigate();

    useEffect(() => {
        const getMyId = async () => {
            try {
                const response = await axios.get('/api/admin/my_id', { withCredentials: true });
                setMyId(response.data);
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
            }
        }
        getMyId();
    }, [navigate]);

    useEffect(() => {
        document.title = 'Admin User Management - HiddN';
        const fetchUsers = async () => {
            try {
                const response = await axios.get('/api/admin/users', { withCredentials: true });
                setUsers(response.data);
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
                setError('Failed to load users.');
                setLoading(false);
            }
        };

        fetchUsers();
    }, [navigate]);

    const handleDeleteUser = async (id: number, email: string) => {
        // Prevent the user from deleting themselves unless they really want to
        let deletingSelf = false;

        if (myId === id) {
            const confirmDelete = window.confirm('Are you sure you want to delete yourself? This action cannot be undone.');
            if (!confirmDelete) {
                return;
            }

            const confirmDelete2 = window.confirm('Are you really sure you want to delete yourself? This action cannot be undone.');
            if (!confirmDelete2) {
                return;
            }

            const confirmDelete4 = window.prompt('If you really really want to delete yourself, please type "I AM AN IDIOT AND I\'D LIKE TO DELETE MYSELF" in the box below.');
            if (confirmDelete4 !== 'I AM AN IDIOT AND I\'D LIKE TO DELETE MYSELF') {
                return;
            }
            deletingSelf = true;
        } else {
            const confirmDelete = window.confirm('Are you sure you want to delete this user ( ' + email + ' )? This action cannot be undone.');
            if (!confirmDelete) {
                return;
            }
        }
        
        try {
            await axios.delete(`/api/admin/users/${id}`, { withCredentials: true });
            alert('User deleted successfully.');
            setUsers(users.filter((user) => user.id !== id));
            if (deletingSelf) {
                navigate('/goodbye');
            }
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    if (err.response.status === 401 || err.response.status === 403) {
                        setError('Unauthorized. Please log in.');
                        navigate('/logout');
                    }
                }
            }
            if (axios.isAxiosError(err) && err.response) {
                setError(err.response.data.message || 'Failed to delete user. Please try again.');
            } else {
                setError('An unexpected error occurred. Please try again.');
            }
        }
    }

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
                                                <Link to={`/admin/user-management/${user.id}`} className="px-4 py-2 text-white bg-blue-500 rounded-md hover:bg-blue-600 dark:bg-blue-600 dark:hover:bg-blue-700">Edit</Link>
                                                <button onClick={() => handleDeleteUser(user.id, user.email)} className="px-4 py-2 ml-2 text-white bg-red-500 rounded-md hover:bg-red-600">Delete</button>
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