import React from 'react';
import { NavLink } from 'react-router-dom';

import ICON from '../assets/hiddn_icon.svg';
import { BookIcon, CreditCardIcon, GearIcon, HomeIcon, PersonIcon, QuestionIcon, SignOutIcon } from '@primer/octicons-react';

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
    icon: React.ReactNode;
}

const SidebarLink: React.FC<SidebarLinkProps> = ({ label, to, icon }) => {
    return (
        <NavLink
            to={to}
            className={({ isActive }) =>
                isActive
                    ? 'p-3 mb-3 rounded-xl bg-hiddn-500 text-white select-none flex items-center'
                    : 'p-3 mb-3 rounded-xl hover:bg-gray-200 select-none flex items-center'
            }
        >
            <span className='items-center ml-3'>{icon}</span>
            <span className="ml-6 text-base">{label}</span>
        </NavLink>
    );
};

const Sidebar: React.FC = React.memo(() => {
    return (
        <div className="flex w-screen h-screen">
            <div className="flex flex-col h-full px-5 py-4 text-black bg-gray-100 min-w-72">
                <NavBarHeader icon={ICON} title="HiddN" />
                <div className="flex flex-col justify-between h-full">
                    <div className="flex flex-col">
                        <SidebarLink to="/user/dashboard" label="Dashboard" icon={<HomeIcon size={24} />} />
                        <SidebarLink to="/user/documentation" label="Documentation" icon={<BookIcon size={24} />} />
                        <SidebarLink to="/user/plan" label="Plan" icon={<GearIcon size={24} />} />
                        <SidebarLink to="/user/transaction" label="Transactions" icon={<CreditCardIcon size={24} />} />
                        <SidebarLink to="/user/support" label="Support" icon={<QuestionIcon size={24} />} />
                        <SidebarLink to="/user/profile" label="Profile" icon={<PersonIcon size={24} />} />
                    </div>
                    <div className="flex flex-col">
                    <SidebarLink to="/logout" label="Logout" icon={<SignOutIcon size={24} />} />
                    </div>
                </div>
            </div>
        </div>
    );
});

export default Sidebar;
