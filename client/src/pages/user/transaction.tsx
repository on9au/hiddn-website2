import React, { useEffect } from 'react';

const Transaction: React.FC = () => {
    useEffect(() => { document.title = 'Hiddn | Transactions'; } );

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">Transaction</h1>
        </div>
    );
};

export default Transaction;
