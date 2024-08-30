import React from 'react';
import { Link } from 'react-router-dom';

interface LinkProps {
    to: string;
    text: string;
}

const CommonLink: React.FC<LinkProps> = ({ to, text }) => {
    return (
        <Link
            to={to}
            className="text-hiddn-500 hover:text-hiddn-400 dark:hover:text-hiddn-600"
        >
            {text}
        </Link>
    );
};

export default CommonLink;
