import React, { useEffect } from 'react';

const Plan: React.FC = () => {
    useEffect(() => { document.title = 'Hiddn | Plans'; } );

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">Plan</h1>
        </div>
    );
};

export default Plan;
