import React from 'react';
import { Link, Outlet } from 'react-router-dom';

const Sidebar: React.FC = () => {
    return (
        <div className="flex">
            <div className="w-64 text-white bg-gray-800">
                <nav>
                    <ul>
                        <li><Link to="/dashboard">Dashboard</Link></li>
                        <li><Link to="/documentation">Documentation</Link></li>
                        <li><Link to="/plan">Plan</Link></li>
                        <li><Link to="/transaction">Transaction</Link></li>
                        <li><Link to="/support">Support</Link></li>
                        <li><Link to="/profile">Profile</Link></li>
                    </ul>
                </nav>
            </div>
            <div className="flex-1 p-6">
                <Outlet />
            </div>
        </div>
    );
};

export default Sidebar;
