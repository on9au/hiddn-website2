import React, { useEffect, useState } from 'react';
import axios from 'axios';
import ReactMarkdown from 'react-markdown';

const Documentation: React.FC = () => {
    const [content, setContent] = useState<string>('');
    const [osList, setOsList] = useState<string[]>([]);
    const [selectedOs, setSelectedOs] = useState<string>('common');
    const [categories, setCategories] = useState<string[]>([]);
    const [selectedCategory, setSelectedCategory] = useState<string>('');
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);

    // Fetch OS options on mount
    useEffect(() => {
        const fetchOsOptions = async () => {
            try {
                const response = await axios.get('/api/documentation/options');
                const availableOsList = response.data.osList;
                setOsList(availableOsList);
                detectOs(availableOsList);
            } catch (err) {
                console.error('Failed to load OS options. Error:', err);
                setError('Failed to load OS options.');
            }
        };

        document.title = 'Hiddn | Documentation';
        fetchOsOptions();
    }, []);

    const detectOs = (availableOsList: string[]) => {
        const userAgent = navigator.userAgent || navigator.vendor;
        let os = 'common';
        if (/windows phone/i.test(userAgent)) {
            os = 'windows';
        } else if (/windows/i.test(userAgent)) {
            os = 'windows';
        } else if (/android/i.test(userAgent)) {
            os = 'android';
        } else if (/iPad|iPhone|iPod/.test(userAgent)) {
            os = 'ios';
        } else if (/mac os/i.test(userAgent)) {
            os = 'macos';
        } else if (/linux/i.test(userAgent)) {
            os = 'linux';
        }

        if (availableOsList.includes(os)) {
            setSelectedOs(os);
        } else {
            setSelectedOs('common');
        }
    };

    // Fetch categories when selectedOs changes
    useEffect(() => {
        if (selectedOs) {
            fetchCategories(selectedOs);
        }
    }, [selectedOs]);

    const fetchCategories = async (os: string) => {
        try {
            const response = await axios.get('/api/documentation/categories', {
                params: { os },
            });
            const fetchedCategories = response.data.categories;
            setCategories(fetchedCategories);
            // Set default category to the first one
            if (fetchedCategories.length > 0) {
                setSelectedCategory(fetchedCategories[0]);
            } else {
                setSelectedCategory('');
                setContent('');
                setError('No categories available for this OS.');
            }
        } catch (err) {
            console.error('Failed to load categories. Error:', err);
            setError('Failed to load categories.');
            setCategories([]);
            setSelectedCategory('');
            setContent('');
        }
    };

    // Fetch documentation when selectedOs or selectedCategory changes
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
                console.error('Failed to load documentation. Error:', err);
                setError('Failed to load documentation.');
                setContent('');
            } finally {
                setLoading(false);
            }
        };

        if (selectedOs && selectedCategory) {
            fetchDocumentation();
        }
    }, [selectedOs, selectedCategory]);

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
                                selectedOs === os ? 'bg-hiddn-500 text-white' : 'dark:bg-gray-700 dark:text-white bg-gray-200 text-black'
                            }`}
                        >
                            {os.charAt(0).toUpperCase() + os.slice(1)}
                        </button>
                    ))}
                </div>
            </div>

            {/* Category Selector */}
            {categories.length > 0 && (
                <div className="mb-4">
                    <h2 className="mb-2 text-2xl">Select Category:</h2>
                    <div className="flex flex-wrap space-x-4">
                        {categories.map((category) => (
                            <button
                                key={category}
                                onClick={() => handleCategoryChange(category)}
                                className={`px-4 py-2 mt-2 rounded ${
                                    selectedCategory === category ? 'bg-hiddn-500 text-white' : 'dark:bg-gray-700 dark:text-white bg-gray-200 text-black'
                                }`}
                            >
                                {category.charAt(0).toUpperCase() + category.slice(1)}
                            </button>
                        ))}
                    </div>
                </div>
            )}

            {/* Content Display */}
            <div className="prose max-w-none dark:prose-invert">
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
