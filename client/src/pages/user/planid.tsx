import React from 'react';
import { useParams } from 'react-router-dom';

const PlanID: React.FC = () => {
    const { id } = useParams<{ id: string }>();

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 dark:text-white">
            <h1 className="text-4xl">PlanID: {id}</h1>
        </div>
    );
};

export default PlanID;
