interface LoginButtonProps {
    content: string;
    handleLogin: () => void;
}

const Loginbutton: React.FC<LoginButtonProps> = ({ content, handleLogin }) => {
    return (
        <button
            className="w-full px-4 py-2 mb-4 text-white bg-hiddn-500 hover:bg-hiddn-400 dark:hover:bg-hiddn-600 rounded-xl"
            onClick={handleLogin}
        >{content}</button>
    );
}

export default Loginbutton;
export type { LoginButtonProps };