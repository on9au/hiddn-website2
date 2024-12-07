import { Route, Routes, Navigate } from 'react-router-dom';
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
import LoginPageRedirect from './pages/loginPageRedirect';
import UserPageRedirect from './pages/user/userPageRedirect';
import UserLayoutAdmin from './components/userlayout_admin';
import AdminDashboard from './pages/admin/dashboard';
import AdminPageRedirect from './pages/admin/adminPageRedirect';
import AdminUserManagement from './pages/admin/userManagement';
import AdminAnnouncementsEditor from './pages/admin/announcementsEditor';
import AdminPlanManager from './pages/admin/planManager';
import UserManagementId from './pages/admin/userManagementid';
import TransactionIDComplete from './pages/user/transactionidcomplete';

const Router = () => {
    return (
        <Routes>
            <Route path="/" element={<LoginPageLayout />}>
                <Route path="login" element={<Login />} />
                <Route path="logout" element={<Logout />} />
                <Route path="register" element={<Register />} />
                <Route path="forgot" element={<Forgot />} />
                <Route path="goodbye" element={<Goodbye />} />
                <Route path="*" element={<LoginPageRedirect />} />
                <Route path="/" element={<Navigate to="/login" />} />
            </Route>
            <Route path="/user" element={<PrivateRoute element={UserLayout} />}>
                <Route path="dashboard" element={<Dashboard />} />
                <Route path="documentation" element={<Documentation />} />
                <Route path="plan" element={<Plan />} />
                <Route path="plan/:id" element={<PlanID />} />
                <Route path="transaction" element={<Transaction />} />
                <Route path="transaction/:id" element={<TransactionID />} />
                <Route path="transaction/:id/complete" element={<TransactionIDComplete />} />
                <Route path="status" element={<Status />} />
                <Route path="support" element={<Support />} />
                <Route path="profile" element={<Profile />} />
                <Route path="delete-account" element={<DeleteProfile />} />
                <Route path="change-password" element={<ChangePassword />} />
                <Route path="*" element={<UserPageRedirect />} />
            </Route>
            <Route path="/admin" element={<PrivateRoute element={UserLayoutAdmin} />}>
                <Route path="dashboard" element={<AdminDashboard />} />
                <Route path="user-management" element={<AdminUserManagement />} />
                <Route path="user-management/:id" element={<UserManagementId />} />
                <Route path="announcement" element={<AdminAnnouncementsEditor />} />
                <Route path="plan" element={<AdminPlanManager/>}/>
                {/*
                <Route path="plan/:id" element={<AdminPlanID/>}/>
                <Route path="transaction" element={<AdminTransaction/>}/>
                <Route path="transaction/:id" element={<AdminTransactionID/>}/>
                <Route path="status" element={<AdminStatus/>}/>
                <Route path="support" element={<AdminSupport/>}/>
                <Route path="profile" element={<AdminProfile/>}/>
                <Route path="change-password" element={<AdminChangePassword/>}/>
                */}
                <Route path="*" element={<AdminPageRedirect />} />
            </Route>
        </Routes>
    )
};

export default Router;
