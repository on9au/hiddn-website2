import axios from "axios";
import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import PageLoading from "../../components/pageloading";

const AdminDashboard: React.FC = () => {
    const navigate = useNavigate();
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        document.title = "Admin - HiddN";
        const amIAdmin = async () => {
            try {
                await axios.get("/api/admin/me", {
                    withCredentials: true,
                });
                setLoading(false);
                setError(null);
            } catch (err) {
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 401 || err.response.status === 403) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        }
                    }
                }
                console.error('Failed to load plan details. Error:', err);
                setError('Failed to load plan details.');
            }
        };

        amIAdmin();
    }, [navigate]);

    const currentTime = new Date();
    let greeting = '';

    if (currentTime.getHours() < 12) {
        greeting = 'Good morning, Admin';
    } else if (currentTime.getHours() < 18) {
        greeting = 'Good afternoon, Admin';
    } else {
        greeting = 'Good evening, Admin';
    }

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">{greeting}.</h1>
            </span>
            <div className="container mx-auto">
                {loading ? (
                    <PageLoading />
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : (
                    <div>
                        <h2 className="text-2xl font-semibold">Admin Dashboard</h2>
                        <p className="text-lg">Welcome to the Admin Dashboard.</p>
                    </div>
                )}
            </div>
        </div>
    );
}

export default AdminDashboard;