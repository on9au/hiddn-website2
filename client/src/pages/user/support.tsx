import React, { useEffect } from 'react';

const Support: React.FC = () => {
    useEffect(() => { document.title = 'Hiddn | Support'; } );

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">Support</h1>
        </div>
    );
};

export default Support;
