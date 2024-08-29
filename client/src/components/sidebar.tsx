import React from 'react';
import { NavLink, Outlet } from 'react-router-dom';

// NavBarHeader Component
interface NavBarHeaderProps {
    icon: string;
    title: string;
}

const NavBarHeader: React.FC<NavBarHeaderProps> = ({ icon, title }) => {
    return (
        <div className="flex items-center mb-3 align-middle">
            <img className="px-1 max-h-16" src={icon} alt="Icon" />
            <h3 className="text-2xl font-bold font-albertsans">{title}</h3>
        </div>
    );
};

// SidebarLink Component
interface SidebarLinkProps {
    label: string;
    to: string;
}

const SidebarLink: React.FC<SidebarLinkProps> = ({ label, to }) => {
    return (
        <NavLink
            to={to}
            className={({ isActive }) =>
                isActive
                    ? 'p-3 mb-3 rounded-xl bg-hiddn-500 text-white select-none'
                    : 'p-3 mb-3 rounded-xl hover:bg-gray-200 select-none'
            }
        >
            {label}
        </NavLink>
    );
};

const Sidebar: React.FC = () => {
    return (
        <div className="flex w-screen h-screen">
            <div className="flex flex-col h-full px-5 py-4 text-black bg-gray-100 min-w-72">
                <NavBarHeader icon="/path/to/icon" title="HiddN" />
                <div className="flex flex-col justify-between h-full">
                    <div className="flex flex-col">
                        <SidebarLink to="/user/dashboard" label="Dashboard" />
                        <SidebarLink to="/user/documentation" label="Documentation" />
                        <SidebarLink to="/user/plan" label="Plan" />
                        <SidebarLink to="/user/transaction" label="Transactions" />
                        <SidebarLink to="/user/support" label="Support" />
                        <SidebarLink to="/user/profile" label="Profile" />
                    </div>
                    <div className="flex flex-col">
                        <NavLink
                            to="/logout"
                            className="p-3 select-none rounded-xl hover:bg-gray-200"
                        >
                            Logout
                        </NavLink>
                    </div>
                </div>
            </div>
            <div className="w-full h-full">
                <Outlet />
            </div>
        </div>
    );
};

export default Sidebar;
