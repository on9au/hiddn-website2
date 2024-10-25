import {Navigate, Route, Routes} from 'react-router-dom';
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
import DeleteProfile from './pages/user/deleteAccount';
import Goodbye from './pages/goodbye';
import ChangePassword from './pages/user/changePassword';

const Router = () => {
    return (
        <Routes>
            <Route path="/" element={<LoginPageLayout/>}>
                <Route path="login" element={<Login/>}/>
                <Route path="logout" element={<Logout/>}/>
                <Route path="register" element={<Register/>}/>
                <Route path="forgot" element={<Forgot/>}/>
                <Route path="goodbye" element={<Goodbye/>}/>
                <Route path="" element={<Navigate to="/login"/>}/>
            </Route>
            <Route path="/user" element={<PrivateRoute element={UserLayout}/>}>
                <Route path="dashboard" element={<Dashboard/>}/>
                <Route path="documentation" element={<Documentation/>}/>
                <Route path="plan" element={<Plan/>}/>
                <Route path="plan/:id" element={<PlanID/>}/>
                <Route path="transaction" element={<Transaction/>}/>
                <Route path="transaction/:id" element={<TransactionID/>}/>
                <Route path="status" element={<Status/>}/>
                <Route path="support" element={<Support/>}/>
                <Route path="profile" element={<Profile/>}/>
                <Route path="delete-account" element={<DeleteProfile/>}/>
                <Route path="change-password" element={<ChangePassword/>}/>
            </Route>
            <Route path="*" element={<Navigate to="/login"/>}/>
        </Routes>
    )
};

export default Router;
