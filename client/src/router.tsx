import { createBrowserRouter, Navigate } from 'react-router-dom';
import Login from './pages/login';
import Register from './pages/register';
import Forgot from './pages/forgot';
import Dashboard from './pages/user/dashboard';
import Logout from './pages/logout';
import UserLayout from './components/userlayout';
import Documentation from './pages/user/documentation';
import Plan from './pages/user/plan';
import PlanID from './pages/user/planid';
import Transaction from './pages/user/transaction';
import Support from './pages/user/support';
import TransactionID from './pages/user/transactionid';
import Profile from './pages/user/profile';

const router = createBrowserRouter([
    // Root, not logged in path
    {
        path: '/',
        element: <Navigate to="/login" />,  // Redirect to /login by default
    },
    {
        path: '/login',
        element: <Login />
    },
    {
        path: '/logout',
        element: <Logout />
    },
    {
        path: '/register',
        element: <Register />
    },
    {
        path: '/forgot',
        element: <Forgot /> 
    },

    // Logged in path
    {
        path: '/user',
        element: <UserLayout />,
        children: [
            {
                path: '',
                element: <Navigate to="/user/dashboard" />,  // Redirect to /dashboard by default
            },
            {
                path: 'dashboard',
                element: <Dashboard />,
            },
            {
                path: 'documentation',
                element: <Documentation />,
            },
            {
                path: 'plan',
                element: <Plan />,
            },
            {
                path: 'plan/:id',
                element: <PlanID />,
            },
            {
                path: 'transaction',
                element: <Transaction />,
            },
            {
                path: 'transaction/:id',
                element: <TransactionID />,
            },
            {
                path: 'support',
                element: <Support />,
            },
            {
                path: 'profile',
                element: <Profile />,
            },
        ]
    },

    // Catch all 404s, redirect to /login
    {
        path: '*',
        element: <Navigate to="/login" /> // Catch-all to handle 404s
    }
])

export default router;
