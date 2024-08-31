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
            <span className="w-full text-left">
                <h1 className="text-4xl">{greeting}.</h1>
            </span>
        </div>
    );
};

export default Dashboard;
