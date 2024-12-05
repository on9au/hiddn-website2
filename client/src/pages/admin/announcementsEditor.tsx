import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { AdminCreateAnnouncement, AnnouncementPayload } from '../../bindings';
import { useNavigate } from 'react-router-dom';
import ReactMarkdown from 'react-markdown';

const AdminAnnouncementsEditor: React.FC = () => {
    const [announcements, setAnnouncements] = useState<AnnouncementPayload[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [newAnnouncement, setNewAnnouncement] = useState<AdminCreateAnnouncement>({ title: '', content: '' });
    const navigate = useNavigate();

    useEffect(() => {
        document.title = 'Admin Announcements Editor - HiddN';
        const fetchAnnouncements = async () => {
            try {
                const response = await axios.get('/api/announcements', { withCredentials: true });
                const reversedAnnouncements = response.data.reverse();
                setAnnouncements(reversedAnnouncements);
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
                setError('Failed to load announcements.');
                setLoading(false);
            }
        };

        fetchAnnouncements();
    }, [navigate]);

    const handleCreateAnnouncement = async () => {
        // If title or content is empty, alert user and return
        if (!newAnnouncement.title || !newAnnouncement.content) {
            alert('Title and content are required.');
            return;
        }
        try {
            const response = await axios.post('/api/admin/announcements', newAnnouncement, { withCredentials: true });
            setAnnouncements([response.data, ...announcements]);
            setNewAnnouncement({ title: '', content: '' });
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    if (err.response.status === 401) {
                        setError('Unauthorized. Please log in.');
                        navigate('/logout');
                    }
                }
            }
            setError('Failed to create announcement.');
        }
    };

    const handleDeleteAnnouncement = async (id: number) => {
        // Alert user to confirm deletion
        const confirmDelete = window.confirm('Are you sure you want to delete this announcement?');
        if (!confirmDelete) {
            return;
        }
        try {
            await axios.delete(`/api/admin/announcements/${id}`, { withCredentials: true });
            setAnnouncements(announcements.filter(announcement => announcement.id !== id));
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    if (err.response.status === 401) {
                        setError('Unauthorized. Please log in.');
                        navigate('/logout');
                    }
                }
            }
            setError('Failed to delete announcement.');
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Announcements Editor</h1>
            </span>
            <div className="container mx-auto">
                <div className="w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                    {loading ? (
                        <p>Loading...</p>
                    ) : error ? (
                        <p className="text-red-500">{error}</p>
                    ) : (
                        <div>
                            <div className="mb-6">
                                <h2 className="mb-2 text-2xl font-semibold">Create New Announcement</h2>
                                <input
                                    type="text"
                                    placeholder="Title"
                                    value={newAnnouncement.title}
                                    onChange={(e) => setNewAnnouncement({ ...newAnnouncement, title: e.target.value })}
                                    className="w-full p-2 mb-4 border rounded dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600"
                                />
                                <textarea
                                    placeholder="Content"
                                    value={newAnnouncement.content}
                                    onChange={(e) => setNewAnnouncement({ ...newAnnouncement, content: e.target.value })}
                                    className="w-full p-2 mb-4 border rounded dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600"
                                />
                                <div className="mt-2 prose max-w-none dark:prose-invert">
                                            <ReactMarkdown>{newAnnouncement.content}</ReactMarkdown>
                                        </div>
                                <button
                                    onClick={handleCreateAnnouncement}
                                    className="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600 dark:bg-green-600 dark:hover:bg-green-700"
                                >
                                    Create
                                </button>
                            </div>
                            <div>
                                <h2 className="mb-2 text-2xl font-semibold">Existing Announcements</h2>
                                {announcements.length > 0 ? announcements.map((announcement) => (
                                    <div key={announcement.id} className="p-4 mb-4 bg-gray-100 rounded shadow-md dark:bg-gray-700">
                                        <h3 className="text-xl font-semibold">{announcement.title}</h3>
                                        <p className="text-sm text-gray-600 dark:text-gray-400">
                                            {new Date(announcement.date).toLocaleDateString()}
                                        </p>
                                        <div className="mt-2 prose max-w-none dark:prose-invert">
                                            <ReactMarkdown>{announcement.content}</ReactMarkdown>
                                        </div>
                                        <button
                                            onClick={() => handleDeleteAnnouncement(announcement.id)}
                                            className="px-4 py-2 mt-2 text-white bg-red-500 rounded-md hover:bg-red-600 dark:bg-red-600 dark:hover:bg-red-700"
                                        >
                                            Delete
                                        </button>
                                    </div>
                                )) : (
                                    <p className="text-gray-700 dark:text-gray-300">No announcements at this time.</p>
                                )}
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default AdminAnnouncementsEditor;