export type AuthStatus =
    | { type: 'Idle' }
    | { type: 'Loading' }
    | { type: 'Success' }
    | { type: 'Error'; message: string };