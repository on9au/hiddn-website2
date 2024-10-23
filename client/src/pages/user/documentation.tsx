import React, { useEffect, useState } from 'react';
import axios from 'axios';
import ReactMarkdown from 'react-markdown';

const Documentation: React.FC = () => {
    const [content, setContent] = useState<string>('');
    const [osList, setOsList] = useState<string[]>([]);
    const [selectedOs, setSelectedOs] = useState<string>('common');
    const [categories, setCategories] = useState<string[]>([]);
    const [selectedCategory, setSelectedCategory] = useState<string>('install');
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchDocumentationOptions = async () => {
            try {
                const response = await axios.get('/api/documentation/options');
                setOsList(response.data.osList);
                setCategories(response.data.categories);
                if (!response.data.osList.includes(selectedOs)) {
                    setSelectedOs('common');
                }
            } catch (err) {
                console.error('Failed to load documentation options. Error: ' + err);
            }
        };
        document.title = 'Hiddn | Documentation';
        detectOs();
        fetchDocumentationOptions();
    }, [selectedOs]);

    useEffect(() => {
        const fetchDocumentation = async () => {
            setLoading(true);
            try {
                const response = await axios.get<string>('/api/documentation', {
                    params: {
                        os: selectedOs,
                        category: selectedCategory,
                    },
                });
                setContent(response.data);
                setError(null);
            } catch (err) {
                setError('Failed to load documentation. Error: ' + err);
                setContent('');
            } finally {
                setLoading(false);
            }
        };
        if (osList.length > 0 && categories.length > 0) {
            fetchDocumentation();
        }
    }, [selectedOs, selectedCategory, osList, categories]);

    const detectOs = () => {
        const userAgent = navigator.userAgent || navigator.vendor;
        if (/windows phone/i.test(userAgent)) {
            setSelectedOs('windows');
        } else if (/windows/i.test(userAgent)) {
            setSelectedOs('windows');
        } else if (/android/i.test(userAgent)) {
            setSelectedOs('android');
        } else if (/iPad|iPhone|iPod/.test(userAgent)) {
            setSelectedOs('ios');
        } else if (/mac os/i.test(userAgent)) {
            setSelectedOs('macos');
        } else if (/linux/i.test(userAgent)) {
            setSelectedOs('linux');
        } else {
            setSelectedOs('common');
        }
    };

    const handleOsChange = (os: string) => {
        setSelectedOs(os);
    };

    const handleCategoryChange = (category: string) => {
        setSelectedCategory(category);
    };

    return (
        <div className="container px-4 py-8 mx-auto">
            <h1 className="mb-8 text-4xl">Documentation</h1>

            {/* OS Selector */}
            <div className="mb-4">
                <h2 className="mb-2 text-2xl">Select Operating System:</h2>
                <div className="flex flex-wrap space-x-4">
                    {osList.map((os) => (
                        <button
                            key={os}
                            onClick={() => handleOsChange(os)}
                            className={`px-4 py-2 mt-2 rounded ${
                                selectedOs === os ? 'bg-blue-500 text-white' : 'bg-gray-200'
                            }`}
                        >
                            {os.charAt(0).toUpperCase() + os.slice(1)}
                        </button>
                    ))}
                </div>
            </div>

            {/* Category Selector */}
            <div className="mb-4">
                <h2 className="mb-2 text-2xl">Select Category:</h2>
                <div className="flex flex-wrap space-x-4">
                    {categories.map((category) => (
                        <button
                            key={category}
                            onClick={() => handleCategoryChange(category)}
                            className={`px-4 py-2 mt-2 rounded ${
                                selectedCategory === category ? 'bg-blue-500 text-white' : 'bg-gray-200'
                            }`}
                        >
                            {category.charAt(0).toUpperCase() + category.slice(1)}
                        </button>
                    ))}
                </div>
            </div>

            {/* Content Display */}
            <div className="prose max-w-none">
                {loading ? (
                    <div className="flex items-center justify-center">
                        <div
                            className="inline-block w-8 h-8 border-4 border-current border-solid rounded-full animate-spin border-r-transparent"
                            role="status"
                        >
                            <span className="sr-only">Loading...</span>
                        </div>
                    </div>
                ) : error ? (
                    <p className="text-red-500">{error}</p>
                ) : (
                    <ReactMarkdown>{content}</ReactMarkdown>
                )}
            </div>
        </div>
    );
};

export default Documentation;
