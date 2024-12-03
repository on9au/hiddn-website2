import React from 'react';
import { NavLink } from 'react-router-dom';

import { CreditCardIcon, GearIcon, HomeIcon, InboxIcon, MegaphoneIcon, MoonIcon, PersonIcon, ServerIcon, SignOutIcon, SunIcon, ThreeBarsIcon, XIcon } from '@primer/octicons-react';
import { Theme } from '../darkmode';

import ICON_LIGHT from '../assets/hiddn_icon.svg';
import ICON_DARK from '../assets/hiddn_icon_dark.svg';

// NavBarHeader Component
interface NavBarHeaderProps {
    icon: string;
    title: string;
}

const NavBarHeader: React.FC<NavBarHeaderProps> = ({ icon, title }) => {
    return (
        <div className="flex items-center mb-3 align-middle select-none">
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
    sidebarToggle?: (arg0: boolean) => void;
}

const SidebarLink: React.FC<SidebarLinkProps> = ({ label, to, icon, sidebarToggle }) => {
    const handleClick = () => {
        if (sidebarToggle && window.innerWidth < 768) {
            sidebarToggle(false);
        }
    };

    return (
        <NavLink
            to={to}
            className={({ isActive }) =>
                isActive
                    ? 'p-3 mb-3 rounded-xl bg-hiddn-500 text-white select-none flex items-center'
                    : 'p-3 mb-3 rounded-xl hover:bg-gray-200 dark:hover:bg-gray-800 select-none flex items-center'
            }
            onClick={handleClick}
        >
            <span className='items-center ml-3'>{icon}</span>
            <span className="pl-px ml-6 text-base">{label}</span>
        </NavLink>
    );
};

interface DarkmodeSidebarProps {
    toggleDarkmode: () => void;
    darkModeState: 'light' | 'dark';
}

const DarkmodeSidebarLink: React.FC<DarkmodeSidebarProps> = ({ toggleDarkmode, darkModeState }) => {
    return (
        <button
            className="flex items-center p-3 mb-3 select-none rounded-xl hover:bg-gray-200 dark:hover:bg-gray-800"
            onClick={toggleDarkmode}
        >
            <span className='items-center ml-3'>{darkModeState === 'light' ? <SunIcon size={24} /> : <MoonIcon size={24} />}</span>
            <span className="pl-px ml-6 text-base">{darkModeState === 'light' ? 'Light Mode' : 'Dark Mode'}</span>
        </button>
    );
}

const SidebarAdmin: React.FC = React.memo(() => {
    const [isSidebarOpen, setIsSidebarOpen] = React.useState(window.innerWidth >= 768);
    const getCurrentTheme = (): boolean => {
        if (typeof window === 'undefined') return false;
        return window.matchMedia("(prefers-color-scheme: dark)").matches;
    };
    const [theme, setTheme] = React.useState(getCurrentTheme() === true ? 'dark' : 'light' as Theme);

    const changeTheme = () => {
        const newTheme = theme === 'light' ? 'dark' : 'light';
        setTheme(newTheme);
        localStorage.setItem('theme', newTheme);
    }

    React.useEffect(() => {
        const savedTheme = localStorage.getItem('theme') as Theme;
        if (savedTheme) {
            setTheme(savedTheme);
        }
    }, []);

    React.useEffect(() => {
        if (theme === 'dark') {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');
        }
    }, [theme]);

    const toggleSidebar = () => {
        setIsSidebarOpen(!isSidebarOpen);
    };

    React.useEffect(() => {
        const handleResize = () => {
            if (window.innerWidth >= 768) {
                setIsSidebarOpen(true);
            } else if (isSidebarOpen && window.innerWidth < 768) { return; } else {
                setIsSidebarOpen(false);
            }
        };

        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, [isSidebarOpen]);

    return (
        <>
            <button
                className="fixed block p-6 text-base text-white rounded-full shadow-lg bg-hiddn-500 md:hidden bottom-8 right-8"
                onClick={toggleSidebar}
            >
                {isSidebarOpen ? <XIcon size={24} /> : <ThreeBarsIcon size={24} />}
            </button>
            <div
                className={`flex flex-col bg-gray-100 dark:bg-gray-900 dark:text-white min-w-72
                md:fixed md:h-full md:top-0 md:left-0 md:px-5 md:py-4 max-md:px-5 max-md:py-4
                ${isSidebarOpen ? 'block' : 'hidden md:block'}
            `}
            >
                <NavBarHeader icon={theme === 'light' ? ICON_LIGHT : ICON_DARK} title="HiddN [ADMIN]" />
                <div className="flex flex-col justify-between h-full">
                    <div className="flex flex-col">
                        {/* Sidebar links */}
                        <SidebarLink to="/admin/dashboard" label="Dashboard" icon={<HomeIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/user-management" label="User Management" icon={<PersonIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/announcement" label="Announcements" icon={<MegaphoneIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/plan" label="Plans" icon={<GearIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/transaction" label="Transactions" icon={<CreditCardIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/status" label="Server Status" icon={<ServerIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/admin/support" label="Support" icon={<InboxIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                    </div>
                    <div className="flex flex-col">
                        <DarkmodeSidebarLink toggleDarkmode={changeTheme} darkModeState={theme} />
                        <SidebarLink to="/user/dashboard" label="Return to Userpage" icon={<PersonIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        <SidebarLink to="/logout" label="Logout" icon={<SignOutIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                    </div>
                </div>
            </div>
        </>
    );
});

export default SidebarAdmin;
