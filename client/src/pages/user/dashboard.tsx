import React from 'react';

const Dashboard: React.FC = () => {
    const currentTime = new Date();
    let greeting = '';

    if (currentTime.getHours() < 12) {
        greeting = 'Good morning';
    } else if (currentTime.getHours() < 18) {
        greeting = 'Good afternoon';
    } else {
        greeting = 'Good evening';
    }

    return (
        <div className="flex flex-col items-center min-h-screen pt-7">
            <span className="w-full mb-6 text-left">
                <h1 className="text-4xl font-semibold">{greeting}.</h1>
            </span>
            <div className="w-full p-4 mb-6 bg-gray-200 dark:bg-gray-800 rounded-2xl">
                <h3 className="mb-3 text-xl font-semibold">Your plan:</h3>
                <p className="mb-px text-base">Expiration: {"TBA"}</p>
                <p className="mb-3 text-base">Status: {"Active"}</p>
                <p>Insert the data usage bar thingy here</p>
            </div>
        </div>
    );
};

export default Dashboard;
