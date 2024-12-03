import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

const AdminPageRedirect = () => {
    useEffect(() => { document.title = '404 - HiddN'; } );

    const navigate = useNavigate();

    useEffect(() => {
        setTimeout(() => {
            navigate('/admin/dashboard');
        } , 1500);
    }
    , [navigate]);

    return (
        <div className="flex flex-col items-center justify-center min-h-screen">
            <h1 className="text-4xl">404</h1>
            <h3>Redirecting to Admin Dashboard...</h3>
        </div>
    );
}

export default AdminPageRedirect;