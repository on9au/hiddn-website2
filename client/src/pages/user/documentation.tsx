import React, { useEffect } from 'react';

const Documentation: React.FC = () => {
    useEffect(() => { document.title = 'Hiddn | Documentation'; } );

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">Documentation</h1>
        </div>
    );
};

export default Documentation;
