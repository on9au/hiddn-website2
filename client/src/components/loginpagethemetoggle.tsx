import { MoonIcon, SunIcon } from "@primer/octicons-react";

interface DarkmodeLoginProps {
    toggleDarkmode: () => void;
    darkModeState: 'light' | 'dark';
}

const LoginPageThemeToggle: React.FC<DarkmodeLoginProps> = ({ toggleDarkmode, darkModeState }) => {
    return (
        // Circular button with a sun or moon icon
        <button
            className="fixed z-50 p-3 bg-white rounded-full shadow-lg bottom-8 max-md:bottom-16 right-8 dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700"
            onClick={toggleDarkmode}
        >
            <span className='items-center'>{darkModeState === 'light' ? <SunIcon size={24} /> : <MoonIcon size={24} />}</span>
        </button>
    );
}

export default LoginPageThemeToggle;