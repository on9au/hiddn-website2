export type AuthStatus =
    | { type: 'Idle' }
    | { type: 'Loading' }
    | { type: 'Success' }
    | { type: 'Error'; message: string };

export type EmailVerifyStatus =
    | { type: 'Idle' }
    | { type: 'Loading' }
    | { type: 'Sent' }
    | { type: 'Error'; message: string };