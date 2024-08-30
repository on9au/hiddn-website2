import React, { useState, useEffect } from 'react';
import { Navigate } from 'react-router-dom';

interface PrivateRouteProps {
    element: React.ComponentType;
}

const PrivateRoute: React.FC<PrivateRouteProps> = ({ element: Component }) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null);

    const useAuth = async () => {
        // Order of checking:
        // 1. localStorage
        // 2. API endpoint

        // Check localStorage to see if we've already checked
        const cached = localStorage.getItem('isAuthenticated');
        if (cached) {
            const { isAuthenticated, expiration } = JSON.parse(cached);
            if (new Date(expiration) > new Date()) {
                setIsAuthenticated(isAuthenticated);
                return;
            }
        }

        // Check api endpoint '/api/is_logged_in' to see if user is authenticated
        try {
            const result = await fetch('/api/is_logged_in');
    
            if (result.status === 200) {
                // User is authenticated
                setIsAuthenticated(true);
                // Delete old cache
                localStorage.removeItem('isAuthenticated');
                // Set new cache
                const expiration = new Date();
                expiration.setMinutes(expiration.getMinutes() + 5);
                localStorage.setItem('isAuthenticated', JSON.stringify({ isAuthenticated: true, expiration }));
                return;
            }
        } catch (error) {
            console.error('Error checking if user is authenticated:', error);
        }

        // User is not authenticated
        setIsAuthenticated(false);
        return;
    };

    useEffect(() => {
        useAuth();
    }, []);

    if (isAuthenticated === null) {
        return null;
    }

    return isAuthenticated ? <Component /> : <Navigate to="/login" />;
};

export default PrivateRoute;
