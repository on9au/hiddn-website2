import React from 'react';
import { Outlet } from 'react-router-dom';
import Sidebar from './sidebar';

const UserLayout: React.FC = () => {
    return (
        <div className="flex w-screen h-screen">
            <Sidebar />
            <div className="w-full h-full">
                <Outlet />
            </div>
        </div>
    );
};

export default UserLayout;
