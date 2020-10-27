import React, { PropsWithChildren, useContext, useEffect } from 'react'
import { Route, RouteProps, useHistory } from 'react-router-dom';
import { AuthContext, defaultAuthContext } from '../Helpers/AuthContext';

export default function AuthedRoute(props: PropsWithChildren<RouteProps>) {
    const authContext = useContext(AuthContext)?.getAuthContext();
    const setAuthContext = useContext(AuthContext)?.setAuthContext;

    useEffect(() => {
        if(!authContext?.authed || !authContext?.token) {
            setAuthContext ? setAuthContext({...defaultAuthContext}) : console.error('AUTH CONTEXT IS UNDEFINED!!');

            window.location.href = '/welcome';
        }
    });

    return (
        <Route {...props}>
            {props.children}
        </Route>
    )
}
