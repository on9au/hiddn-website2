import { createBrowserRouter, Navigate } from 'react-router-dom';
import Login from './components/login';
import Sidebar from './components/sidebar';
// import Register from './components/register';
// import Forgot from './components/forgot';

// import Dashboard from './components/dashboard';
// import Documentation from './components/documentation';
// import Plan from './components/plan';
// import PlanID from './components/planid';
// import Transaction from './components/transaction';
// import TransactionID from './components/transactionid';
// import Support from './components/support';
// import Profile from './components/profile';
// import Sidebar from './components/sidebar';

// const AppRouter: React.FC = () => {
//     return (
//         <BrowserRouter>
//             <Routes>
//                 {/* Login Section */}
//                 <Route path="/login" element={<Login />} />
//                 {/* <Route path="/register" element={<Register />} />
//                 <Route path="/forgot" element={<Forgot />} /> */}

//                 {/* User Section */}
//                 {/* <Route path="/dashboard" element={<Dashboard />} />
//                 <Route path="/documentation" element={<Documentation />} />
//                 <Route path="/plan/:id" element={<PlanID />} />
//                 <Route path="/plan" element={<Plan />} />
//                 <Route path="/transaction/:id" element={<TransactionID />} />
//                 <Route path="/transaction" element={<Transaction />} />
//                 <Route path="/support" element={<Support />} />
//                 <Route path="/profile" element={<Profile />} /> */}

//                 {/* Redirect to login by default */}
//                 <Route path="*" element={<Navigate to="/login" />} />
//             </Routes>
//         </BrowserRouter>
//     );
// };

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

    // Logged in path
    {
        path: '/user',
        element: <Sidebar />,
        children: [
            // {
            //     path: 'dashboard',
            //     element: <Dashboard />,
            // },
            // {
            //     path: 'documentation',
            //     element: <Documentation />,
            // },
            // {
            //     path: 'plan',
            //     element: <Plan />,
            // },
            // {
            //     path: 'plan/:id',
            //     element: <PlanID />,
            // },
            // {
            //     path: 'transaction',
            //     element: <Transaction />,
            // },
            // {
            //     path: 'transaction/:id',
            //     element: <TransactionID />,
            // },
            // {
            //     path: 'support',
            //     element: <Support />,
            // },
            // {
            //     path: 'profile',
            //     element: <Profile />,
            // },
        ]
    },

    // Catch all 404s, redirect to /login
    {
        path: '*',
        element: <Navigate to="/login" /> // Catch-all to handle 404s
    }
])

export default router;
