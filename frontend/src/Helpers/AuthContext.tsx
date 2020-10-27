import React, { useContext, createContext, useEffect, useState } from 'react';
import { IAuthContext, TSetAuthContext } from '../Interfaces/IAuthContext';

const defaultAuthContext = {
    authed: false,
    token: '',
    userId: '',
    loading: true
}
  
const AuthContext = createContext<{
    getAuthContext: () => IAuthContext,
    setAuthContext: (authContext: IAuthContext) => void
} | undefined>(undefined);

function AuthProvider({children}:any) {
    const [authContext, setAuthContext] = useState<IAuthContext>({
        ...defaultAuthContext
    });

    return (
        <AuthContext.Provider value={{
            getAuthContext: () => authContext,
            setAuthContext: setAuthContext
        }}>
            {children}
        </AuthContext.Provider>
    )
}

export { AuthProvider, AuthContext, defaultAuthContext }
