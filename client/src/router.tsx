import { Navigate, Route, Routes } from 'react-router-dom';
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
import LoginPageLayout from './components/loginpagelayout';
import PrivateRoute from './components/privateroute';
import Status from './pages/user/status';

const Router = () => {
    return (
        <Routes>
            <Route path="/" element={<LoginPageLayout />}>
                <Route path="login" element={<Login />} />
                <Route path="logout" element={<Logout />} />
                <Route path="register" element={<Register />} />
                <Route path="forgot" element={<Forgot />} />
                <Route path="" element={<Navigate to="/login" />} />
            </Route>
            <Route path="/user" element={<PrivateRoute element={UserLayout} />}>
                <Route path="dashboard" element={<Dashboard />} />
                <Route path="documentation" element={<Documentation />} />
                <Route path="plan" element={<Plan />} />
                <Route path="plan/:id" element={<PlanID />} />
                <Route path="transaction" element={<Transaction />} />
                <Route path="transaction/:id" element={<TransactionID />} />
                <Route path="status" element={<Status />} />
                <Route path="support" element={<Support />} />
                <Route path="profile" element={<Profile />} />
            </Route>
            <Route path="*" element={<Navigate to="/login" />} />
        </Routes>
    )
    // // Root, not logged in path
    // {
    //     path: '/',
    //     element: <LoginPageLayout />,
    //     children: [
    //         {
    //             path: '',
    //             element: <Navigate to="/login" />,  // Redirect to /login by default
    //         },
    //         {
    //             path: '/login',
    //             element: <Login />
    //         },
    //         {
    //             path: '/logout',
    //             element: <Logout />
    //         },
    //         {
    //             path: '/register',
    //             element: <Register />
    //         },
    //         {
    //             path: '/forgot',
    //             element: <Forgot />
    //         },
    //         {
    //             path: '',
    //             element: <Navigate to="/login" />,  // Redirect to /login by default
    //         }
    //     ]
    // },

    // // Logged in path
    // {
    //     path: '/user',
    //     element: <PrivateRoute element={UserLayout} />,  // Protect the UserLayout with PrivateRoute
    //     children: [
    //         {
    //             path: 'dashboard',
    //             element: <Dashboard />,
    //         },
    //         {
    //             path: 'documentation',
    //             element: <Documentation />,
    //         },
    //         {
    //             path: 'plan',
    //             element: <Plan />,
    //         },
    //         {
    //             path: 'plan/:id',
    //             element: <PlanID />,
    //         },
    //         {
    //             path: 'transaction',
    //             element: <Transaction />,
    //         },
    //         {
    //             path: 'transaction/:id',
    //             element: <TransactionID />,
    //         },
    //         {
    //             path: 'support',
    //             element: <Support />,
    //         },
    //         {
    //             path: 'profile',
    //             element: <Profile />,
    //         },
    //     ]
    // },

    // // Catch all 404s, redirect to /login
    // {
    //     path: '*',
    //     element: <Navigate to="/login" /> // Catch-all to handle 404s
    // }
};

export default Router;
