import React from 'react';
import { Outlet } from 'react-router-dom';
import LoginPageThemeToggle from './loginpagethemetoggle';
import { Theme } from '../darkmode';
import FrontpageHeader from './frontpageheader';

import ICON_LIGHT from '../assets/hiddn_icon.svg';
import ICON_DARK from '../assets/hiddn_icon_dark.svg';

const LoginPageLayout: React.FC = () => {
    const [theme, setTheme] = React.useState('light' as Theme);

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
    
    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100 dark:bg-gray-900 dark:text-white">
            <LoginPageThemeToggle toggleDarkmode={changeTheme} darkModeState={theme} />
            <FrontpageHeader icon={theme === 'light' ? ICON_LIGHT : ICON_DARK} title="HiddN" />
            
            <Outlet />
        </div>
    );
};

export default LoginPageLayout;
