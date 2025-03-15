import React, { useEffect } from 'react';

const Support: React.FC = () => {
    useEffect(() => { document.title = 'Support - HiddN'; });

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">For the time being, please contact <a href="mailto:contact@hiddnvpn.net">contact@hiddnvpn.net</a></h1>
        </div>
    );
};

export default Support;
