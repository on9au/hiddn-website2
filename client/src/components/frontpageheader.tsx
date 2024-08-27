import React from 'react';

interface FrontpageHeaderProps {
    icon: string;
    title: string;
}

const FrontpageHeader: React.FC<FrontpageHeaderProps> = ({ icon, title }) => {
    return (
        <div className="flex flex-col items-center">
            <img className="max-h-16" src={icon} alt={title} />
            <h3 className="mb-8 text-2xl font-bold font-albertsans">{title}</h3>
        </div>
    );
};

export default FrontpageHeader;
