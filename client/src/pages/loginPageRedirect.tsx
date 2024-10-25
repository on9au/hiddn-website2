import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

const LoginPageRedirect = () => {
    useEffect(() => { document.title = '404 - HiddN'; } );

    const navigate = useNavigate();

    useEffect(() => {
        setTimeout(() => {
            navigate('/login');
        } , 500);
    }
    , [navigate]);

    return (
        <div className="flex flex-col items-center min-h-screen px-5 py-32 bg-gray-100 max-md:py-16 dark:bg-gray-900 dark:text-white">
            <h1 className="mb-4 text-4xl">404</h1>
            <h3>Redirecting to Login Page...</h3>
        </div>
    );
}

export default LoginPageRedirect;