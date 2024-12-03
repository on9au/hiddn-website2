import React from 'react';
import { Outlet } from 'react-router-dom';
import SidebarAdmin from './sidebar_admin';

const UserLayoutAdmin: React.FC = () => {
    return (
        <div className="flex flex-col w-screen h-screen bg-gray-100 md:flex-row dark:bg-gray-900">
            {/* Use md:flex-row to switch to row layout on medium screens and above */}
            <SidebarAdmin />
            <div className="flex flex-col items-center justify-center w-full h-full px-5 bg-gray-100 dark:bg-gray-900 dark:text-white md:ml-72">
                {/* Add md:ml-72 to offset the main content by the width of the sidebar on larger screens */}
                <div className="w-full h-full overflow-y-auto max-w-7xl">
                    <Outlet />
                </div>
            </div>
        </div>
    );
};

export default UserLayoutAdmin;
