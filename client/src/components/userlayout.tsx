import React from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from './sidebar';

const UserLayout: React.FC = () => {
    return (
        <div className="flex flex-col w-screen h-screen md:flex-row">
            <Sidebar />
            <div className="flex flex-col items-center justify-center w-full h-full px-5 bg-gray-100 dark:bg-gray-900 dark:text-white">
                <div className="w-full h-full max-w-7xl">
                    <Outlet />
                </div>
            </div>
        </div>
    );
};

export default UserLayout;
