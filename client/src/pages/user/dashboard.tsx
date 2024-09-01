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
            <span className="w-full text-left mb-6">
                <h1 className="text-4xl font-semibold">{greeting}.</h1>
            </span>
            <div className="w-full mb-6 bg-gray-200 dark:bg-slate-800 p-4 rounded-2xl">
                <h3 className="text-xl font-semibold mb-3">Your plan:</h3>
                <p className="text-base mb-px">Expiration: {"TBA"}</p>
                <p className="text-base mb-3">Status: {"Active"}</p>
                <p>Insert the data usage bar thingy here</p>
            </div>
        </div>
    );
};

export default Dashboard;
