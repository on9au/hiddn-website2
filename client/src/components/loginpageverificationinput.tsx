import React from 'react';
import TextInput from './logintextinput';
import { EmailVerifyStatus } from '../auth';

interface VerificationInputProps {
    value: string;
    onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
    onSendCode: () => Promise<void>;
    verifyStatus: EmailVerifyStatus;
    verifyTimeout: number;
}

const VerificationInput: React.FC<VerificationInputProps> = ({
    value,
    onChange,
    onSendCode,
    verifyStatus,
    verifyTimeout,
}) => {
    return (
        <div className="relative w-full">
            <TextInput
                type="text"
                placeholder="Verification Code"
                is_last_position={false}
                value={value}
                onChange={onChange}
                maxLength={20}
            />
            <button
                className={`absolute right-0 w-20 px-4 py-2 text-white border rounded-xl ${verifyStatus.type === 'Idle' || verifyStatus.type === 'Error' ? 'bg-hiddn-500 hover:bg-hiddn-400 border-hiddn-500 hover:border-hiddn-400' : 'bg-gray-400 border-gray-400 cursor-not-allowed'}`}
                onClick={async () => {
                    if (verifyStatus.type === 'Idle' || verifyStatus.type === 'Error') {
                        await onSendCode();
                    }
                }}
                disabled={verifyStatus.type !== 'Idle' && verifyStatus.type !== 'Error'}
            >
                {verifyStatus.type === 'Idle' ? 'Send' : verifyStatus.type === 'Error' ? 'Send' : verifyStatus.type === 'Loading' ? '...' : verifyTimeout.toString()}
            </button>
        </div>
    );
};

export default VerificationInput;
