import React, { useEffect, useState } from 'react';
import axios from 'axios';
import {
    FaDesktop,
    FaMobileAlt,
    FaApple,
    FaLinux,
    FaWindows,
    FaAndroid,
} from 'react-icons/fa';
import ReactMarkdown from 'react-markdown';
import { useNavigate } from 'react-router-dom';

const Documentation: React.FC = () => {
    const [content, setContent] = useState<string>('');
    const [osList, setOsList] = useState<string[]>([]);
    const [selectedOs, setSelectedOs] = useState<string>('common');
    const [categories, setCategories] = useState<string[]>([]);
    const [selectedCategory, setSelectedCategory] = useState<string>('');
    const [loading, setLoading] = useState<boolean>(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();

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

        document.title = 'Documentation - HiddN';
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
                    setError('No documentation available for this OS.');
                }
            } catch (err) {
                
                if (axios.isAxiosError(err)) {
                    if (err.response) {
                        if (err.response.status === 401) {
                            setError('Unauthorized. Please log in.');
                            navigate('/logout');
                        }
                    }
                }
                console.error('Failed to load categories. Error:', err);
                setError('Failed to load categories.');
                setCategories([]);
                setSelectedCategory('');
                setContent('');
            }
        };

        if (selectedOs) {
            fetchCategories(selectedOs);
        }
    }, [navigate, selectedOs]);

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

    // Helper function to get OS icon
    const getOsIcon = (os: string) => {
        switch (os) {
            case 'windows':
                return <FaWindows className="inline-block mr-2" />;
            case 'macos':
                return <FaApple className="inline-block mr-2" />;
            case 'linux':
                return <FaLinux className="inline-block mr-2" />;
            case 'android':
                return <FaAndroid className="inline-block mr-2" />;
            case 'ios':
                return <FaMobileAlt className="inline-block mr-2" />;
            default:
                return <FaDesktop className="inline-block mr-2" />;
        }
    };

    return (
        <div className="flex flex-col pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Documentation</h1>
            </span>
            <div className="container flex flex-col mx-auto lg:flex-row">
                {/* Sidebar */}
                <div className="flex-shrink-0 w-full mb-8 lg:w-1/4 lg:pr-8 lg:mb-0">
                    {/* OS Selector */}
                    <div className="p-4 mb-6 bg-white rounded-lg shadow-md dark:bg-gray-800">
                        <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                            Operating System
                        </h2>
                        <div className="space-y-2">
                            {osList.map((os) => (
                                <button
                                    key={os}
                                    onClick={() => handleOsChange(os)}
                                    className={`flex items-center w-full px-3 py-2 text-left rounded-md focus:outline-none transition-colors ${selectedOs === os
                                            ? 'bg-hiddn-500 text-white'
                                            : 'text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700'
                                        }`}
                                >
                                    {getOsIcon(os)}
                                    <span>{os.charAt(0).toUpperCase() + os.slice(1)}</span>
                                </button>
                            ))}
                        </div>
                    </div>

                    {/* Category Selector */}
                    {categories.length > 0 && (
                        <div className="p-4 bg-white rounded-lg shadow-md dark:bg-gray-800">
                            <h2 className="mb-4 text-2xl font-semibold text-gray-800 dark:text-gray-200">
                                Categories
                            </h2>
                            <div className="space-y-2">
                                {categories.map((category) => (
                                    <button
                                        key={category}
                                        onClick={() => handleCategoryChange(category)}
                                        className={`w-full px-3 py-2 text-left rounded-md focus:outline-none transition-colors ${selectedCategory === category
                                                ? 'bg-hiddn-500 text-white'
                                                : 'text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700'
                                            }`}
                                    >
                                        {category.charAt(0).toUpperCase() + category.slice(1)}
                                    </button>
                                ))}
                            </div>
                        </div>
                    )}
                </div>

                {/* Content Display */}
                <div
                    className={`w-full p-6 bg-white rounded-lg shadow-md dark:bg-gray-800 ${categories.length > 0 ? 'lg:w-3/4' : 'lg:w-full'
                        } lg:overflow-y-auto lg:max-h-[80vh]`}
                >
                    {loading ? (
                        <div className="flex items-center justify-center h-full">
                            <div
                                className="inline-block w-12 h-12 border-4 border-current border-blue-500 border-solid rounded-full animate-spin border-r-transparent"
                                role="status"
                            >
                                <span className="sr-only">Loading...</span>
                            </div>
                        </div>
                    ) : error ? (
                        <p className="text-red-500">{error}</p>
                    ) : (
                        <div className="prose max-w-none dark:prose-invert">
                            <ReactMarkdown>{content}</ReactMarkdown>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Documentation;
