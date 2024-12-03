import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const AdminDocumentationEditor: React.FC = () => {
    const [docs, setDocs] = useState<{ [key: string]: { [key: string]: string } }>({});
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [newDoc, setNewDoc] = useState({ os: '', category: '', content: '' });
    const navigate = useNavigate();

    useEffect(() => {
        const fetchDocs = async () => {
            try {
                const response = await axios.get('/api/documentation', { withCredentials: true });
                setDocs(response.data);
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
                setError('Failed to load documentation.');
                setLoading(false);
            }
        };

        fetchDocs();
    }, [navigate]);

    const handleCreateDoc = async () => {
        try {
            await axios.post('/api/documentation', newDoc, { withCredentials: true });
            setDocs({ ...docs, [newDoc.os]: { ...docs[newDoc.os], [newDoc.category]: newDoc.content } });
            setNewDoc({ os: '', category: '', content: '' });
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    if (err.response.status === 401) {
                        setError('Unauthorized. Please log in.');
                        navigate('/logout');
                    }
                }
            }
            setError('Failed to create documentation.');
        }
    };

    const handleDeleteDoc = async (os: string, category: string) => {
        try {
            await axios.delete(`/api/documentation?os=${os}&category=${category}`, { withCredentials: true });
            const updatedDocs = { ...docs };
            delete updatedDocs[os][category];
            setDocs(updatedDocs);
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    if (err.response.status === 401) {
                        setError('Unauthorized. Please log in.');
                        navigate('/logout');
                    }
                }
            }
            setError('Failed to delete documentation.');
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Documentations Editor</h1>
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
                                <h2 className="text-2xl font-semibold">Create New Documentation</h2>
                                <input
                                    type="text"
                                    placeholder="OS"
                                    value={newDoc.os}
                                    onChange={(e) => setNewDoc({ ...newDoc, os: e.target.value })}
                                    className="w-full p-2 mb-4 border rounded"
                                />
                                <input
                                    type="text"
                                    placeholder="Category"
                                    value={newDoc.category}
                                    onChange={(e) => setNewDoc({ ...newDoc, category: e.target.value })}
                                    className="w-full p-2 mb-4 border rounded"
                                />
                                <textarea
                                    placeholder="Content"
                                    value={newDoc.content}
                                    onChange={(e) => setNewDoc({ ...newDoc, content: e.target.value })}
                                    className="w-full p-2 mb-4 border rounded"
                                />
                                <button
                                    onClick={handleCreateDoc}
                                    className="px-4 py-2 text-white bg-green-500 rounded-md hover:bg-green-600"
                                >
                                    Create
                                </button>
                            </div>
                            <div>
                                <h2 className="text-2xl font-semibold">Existing Documentation</h2>
                                {Object.keys(docs).map((os) => (
                                    <div key={os}>
                                        <h3 className="text-xl font-semibold">{os}</h3>
                                        {Object.keys(docs[os]).map((category) => (
                                            <div key={category} className="p-4 mb-4 bg-gray-100 rounded shadow-md dark:bg-gray-700">
                                                <h4 className="text-lg font-semibold">{category}</h4>
                                                <p>{docs[os][category]}</p>
                                                <button
                                                    onClick={() => handleDeleteDoc(os, category)}
                                                    className="px-4 py-2 mt-2 text-white bg-red-500 rounded-md hover:bg-red-600"
                                                >
                                                    Delete
                                                </button>
                                            </div>
                                        ))}
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default AdminDocumentationEditor;