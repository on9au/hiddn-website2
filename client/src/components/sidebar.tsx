import React from 'react';
import { NavLink } from 'react-router-dom';

import ICON from '../assets/hiddn_icon.svg';
import { BookIcon, CreditCardIcon, GearIcon, HomeIcon, MoonIcon, PersonIcon, QuestionIcon, SignOutIcon, SunIcon, ThreeBarsIcon, XIcon } from '@primer/octicons-react';

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
                    : 'p-3 mb-3 rounded-xl hover:bg-gray-200 select-none flex items-center'
            }
            onClick={handleClick}
        >
            <span className='items-center ml-3'>{icon}</span>
            <span className="ml-6 pl-px text-base">{label}</span>
        </NavLink>
    );
};

interface DarkmodeSidebarProps {
    isDarkmode: boolean;
    toggleDarkmode: () => void;
}

const DarkmodeSidebarLink: React.FC<DarkmodeSidebarProps> = ({ isDarkmode, toggleDarkmode }) => {
    return (
        <button
            className="p-3 mb-3 rounded-xl hover:bg-gray-200 select-none flex items-center"
            onClick={toggleDarkmode}
        >
            <span className='items-center ml-3'>{isDarkmode ? <SunIcon size={24} /> : <MoonIcon size={24} />}</span>
            <span className="ml-6 pl-px text-base">{isDarkmode ? "Light Mode" : "Dark Mode"}</span>
        </button>
    );
}

const Sidebar: React.FC = React.memo(() => {
    const [isSidebarOpen, setIsSidebarOpen] = React.useState(window.innerWidth >= 768);

    const toggleSidebar = () => {
        setIsSidebarOpen(!isSidebarOpen);
    };

    React.useEffect(() => {
        const handleResize = () => {
            if (window.innerWidth >= 768) {
                setIsSidebarOpen(true);
            } else {
                setIsSidebarOpen(false);
            }
        };

        window.addEventListener('resize', handleResize);

        return () => {
            window.removeEventListener('resize', handleResize);
        };
    }, []);

    return (
        <>
            <button className="fixed block p-6 bg-hiddn-500 rounded-full md:hidden bottom-8 right-8 text-white text-base" onClick={toggleSidebar}>
                {isSidebarOpen ? <XIcon size={24} /> : <ThreeBarsIcon size={24} />}
            </button>
            {isSidebarOpen && (
                <div className="flex flex-col h-full px-5 py-4 text-black bg-gray-100 min-w-72">
                    <NavBarHeader icon={ICON} title="HiddN" />
                    <div className="flex flex-col justify-between h-full">
                        <div className="flex flex-col">
                            <SidebarLink to="/user/dashboard" label="Dashboard" icon={<HomeIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                            <SidebarLink to="/user/documentation" label="Documentation" icon={<BookIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                            <SidebarLink to="/user/plan" label="Plan" icon={<GearIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                            <SidebarLink to="/user/transaction" label="Transactions" icon={<CreditCardIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                            <SidebarLink to="/user/support" label="Support" icon={<QuestionIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                            <SidebarLink to="/user/profile" label="Profile" icon={<PersonIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        </div>
                        <div className="flex flex-col">
                            <DarkmodeSidebarLink isDarkmode={false} toggleDarkmode={() => { }} />
                            <SidebarLink to="/logout" label="Logout" icon={<SignOutIcon size={24} />} sidebarToggle={setIsSidebarOpen} />
                        </div>
                    </div>
                </div>
            )}
        </>
    );
});

export default Sidebar;
