import React from 'react';
import { useNavigate } from 'react-router-dom';
import { UserProfilePayload } from '../../bindings';

type FetchUserStatusEnum =
    | { 'status': 'loading' }
    | { 'status': 'success' }
    | { 'status': 'error', 'message': string }

const Profile: React.FC = () => {
    const navigate = useNavigate();
    const [userProfile, setUserProfile] = React.useState(null as UserProfilePayload | null);
    const [fetchServerStatus, setFetchServerStatus] = React.useState({ 'status': 'loading' } as FetchUserStatusEnum);

    const fetchUserProfile = async () => {
        try {
            const result = await fetch(`/api/me`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            });
            if (result.ok) {
                setFetchServerStatus({ 'status': 'success' });
                const json = await result.json();
                setUserProfile(json as UserProfilePayload);
            } else if (result.status === 401) {
                setFetchServerStatus({ 'status': 'error', message: 'Unauthorized. Please log in.' });
                navigate('/logout');
            } else {
                setFetchServerStatus({ 'status': 'error', message: result.statusText });
            }
        } catch (error) {
            setFetchServerStatus({ 'status': 'error', message: 'Failed to fetch user profile. Try again later. Error: ' + error });
        }
    };

    React.useEffect(() => {
        fetchUserProfile();
    }, []);

    return (
        <div className="flex flex-col items-center min-h-screen pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">Profile</h1>
            </span>
            <div className="w-full p-4 mb-6 bg-gray-200 dark:bg-gray-800 rounded-2xl">
                {fetchServerStatus.status === 'loading' && (
                    <div className="flex justify-center">
                        <div
                            className="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]"
                            role="status">
                            <span
                                className="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                            >Loading...</span>
                        </div>
                    </div>
                )}
                {fetchServerStatus.status === 'error' && (
                    <p className="text-red-500">Error: {fetchServerStatus.message}</p>
                )}
                {fetchServerStatus.status === 'success' && (
                    <>
                        <h3 className="mb-3 text-xl font-semibold">Hello, {userProfile?.email}</h3>
                        <p className="mb-px text-base">Created at: {userProfile?.created_at}</p>
                    </>
                )}
            </div>
            <div className="w-full mb-6">
                hi
            </div>
        </div>
    );
};

export default Profile;
