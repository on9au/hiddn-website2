import React from 'react';
import { Outlet, useNavigate } from 'react-router-dom';
import LoginPageThemeToggle from './loginpagethemetoggle';
import { Theme } from '../darkmode';
import FrontpageHeader from './frontpageheader';

import ICON_LIGHT from '../assets/hiddn_icon.svg';
import ICON_DARK from '../assets/hiddn_icon_dark.svg';
import { isUserAuth } from '../auth';
import PageLoading from './pageloading';

const LoginPageLayout: React.FC = () => {
    const getCurrentTheme = (): boolean => {
        if (typeof window === 'undefined') return false; 
        return window.matchMedia("(prefers-color-scheme: dark)").matches;
    };
    const [theme, setTheme] = React.useState(getCurrentTheme() === true ? 'dark' : 'light' as Theme);
    const [isAuthenticated, setIsAuthenticated] = React.useState<boolean | null>(null);
    const navigate = useNavigate();

    const changeTheme = () => {
        const newTheme = theme === 'light' ? 'dark' : 'light';
        setTheme(newTheme);
        localStorage.setItem('theme', newTheme);
    }

    React.useEffect(() => {
        if (typeof window !== 'undefined') {
            const savedTheme = localStorage.getItem('theme') as Theme;
            if (savedTheme) {
                setTheme(savedTheme);
            }
        }
    }, []);

    React.useEffect(() => {
        if (typeof window !== 'undefined') {
            if (theme === 'dark') {
                document.documentElement.classList.add('dark');
            } else {
                document.documentElement.classList.remove('dark');
            }
        }

    }, [theme]);

    React.useEffect(() => {
        isUserAuth(setIsAuthenticated);
    }, []);

    if (isAuthenticated === null) {
        return <PageLoading />;
    }

    // If user is authenticated, show the dashboard page
    if (isAuthenticated) {
        if (typeof window !== 'undefined') {
            if (window.location.pathname !== '/logout') {
                navigate('/user/dashboard');
            }
        }
    }

    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100 dark:bg-gray-900 dark:text-white">
            <LoginPageThemeToggle toggleDarkmode={changeTheme} darkModeState={theme} />
            <FrontpageHeader icon={theme === 'light' ? ICON_LIGHT : ICON_DARK} title="HiddN" />

            <Outlet />
        </div>
    );
};

export default LoginPageLayout;
