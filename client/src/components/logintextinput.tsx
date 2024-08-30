import React from 'react';

interface TextInputProps {
    type: string;
    placeholder: string;
    value: string;
    is_last_position: boolean,
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    maxLength?: number;
    onKeyDown?: (e: React.KeyboardEvent<HTMLInputElement>) => void;
}

const TextInput: React.FC<TextInputProps> = ({
    type,
    placeholder,
    value,
    is_last_position,
    onChange,
    maxLength,
    onKeyDown,
}) => {
    return (
        <input
            className={`w-full px-4 py-2 border border-gray-300 outline-none dark:border-gray-700 dark:bg-gray-800 dark:hover:border-hiddn-600 hover:border-hiddn-200 focus:border-hiddn-500 rounded-xl ${is_last_position ? 'mb-4' : 'mb-2'}`}
            type={type}
            placeholder={placeholder}
            value={value}
            onChange={onChange}
            maxLength={maxLength}
            onKeyDown={onKeyDown}
        />
    );
};

export default TextInput;
