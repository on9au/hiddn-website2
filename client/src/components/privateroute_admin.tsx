import React, { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { isUserAdmin } from '../auth';
import PageLoading from './pageloading';

interface PrivateRouteProps {
    element: React.ComponentType;
}

const PrivateRouteAdmin: React.FC<PrivateRouteProps> = ({ element: Component }) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null);

    useEffect(() => {
        isUserAdmin(setIsAuthenticated);
    }, []);

    if (isAuthenticated === null) {
        return <PageLoading />;
    }

    return isAuthenticated ? <Component /> : <Navigate to="/login" />;
};

export default PrivateRouteAdmin;
