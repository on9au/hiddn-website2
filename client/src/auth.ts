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

export type LogoutStatus =
    | { type: 'Idle' }
    | { type: 'LoggingOut' }
    | { type: 'Success' }
    | { type: 'Error'; message: string };

// Takes in state of type boolean|null and sets it to true if user is authenticated, false if not authenticated.
export const isUserAuth = async (setIsAuthenticated: React.Dispatch<React.SetStateAction<boolean|null>>, ignore_cache: boolean = false, cache_valid_minutes: number = 5) => {
    // Order of checking:
    // 1. localStorage
    // 2. API endpoint

    // Check localStorage to see if we've already checked
    // If ignore_cache is true, skip this step
    if (!ignore_cache) {
        const cache = localStorage.getItem('isAuthenticated');
        if (cache) {
            const { isAuthenticated, expiration } = JSON.parse(cache);
            if (new Date(expiration) > new Date()) {
                setIsAuthenticated(isAuthenticated);
                return;
            }
        }
    }

    // Check api endpoint '/api/is_logged_in' to see if user is authenticated
    try {
        const result = await fetch('/api/is_logged_in');

        if (result.status === 200) {
            // User is authenticated
            setIsAuthenticated(true);
            // Delete old cache
            localStorage.removeItem('isAuthenticated');
            // Set new cache
            const expiration = new Date();
            expiration.setMinutes(expiration.getMinutes() + cache_valid_minutes);
            localStorage.setItem('isAuthenticated', JSON.stringify({ isAuthenticated: true, expiration }));
            return;
        }
    } catch (error) {
        console.error('Error checking if user is authenticated:', error);
    }

    // User is not authenticated
    setIsAuthenticated(false);
    return;
};

export const isUserAdmin = async (setIsAdmin: React.Dispatch<React.SetStateAction<boolean|null>>, ignore_cache: boolean = false, cache_valid_minutes: number = 5) => {
    // Order of checking:
    // 1. localStorage
    // 2. API endpoint

    // Check localStorage to see if we've already checked
    // If ignore_cache is true, skip this step
    if (!ignore_cache) {
        const cache = localStorage.getItem('isAdmin');
        if (cache) {
            const { isAdmin, expiration } = JSON.parse(cache);
            if (new Date(expiration) > new Date()) {
                setIsAdmin(isAdmin);
                return;
            }
        }
    }

    // Check api endpoint '/api/admin/me' to see if user is an admin
    try {
        const result = await fetch('/api/admin/me');

        if (result.status === 200) {
            // User is an admin
            setIsAdmin(true);
            // Delete old cache
            localStorage.removeItem('isAdmin');
            // Set new cache
            const expiration = new Date();
            expiration.setMinutes(expiration.getMinutes() + cache_valid_minutes);
            localStorage.setItem('isAdmin', JSON.stringify({ isAdmin: true, expiration }));
            return;
        }
    } catch (error) {
        console.error('Error checking if user is an admin:', error);
    }

    // User is not an admin
    setIsAdmin(false);
    return;
}