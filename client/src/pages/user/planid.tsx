import React, { useEffect } from 'react';
import { useParams } from 'react-router-dom';

const PlanID: React.FC = () => {
    const { id } = useParams<{ id: string }>();

    useEffect(() => { document.title = 'Hiddn | Plan ' + id; } );

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">PlanID: {id}</h1>
        </div>
    );
};

export default PlanID;
