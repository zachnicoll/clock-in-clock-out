import React, { useContext, useEffect } from 'react';
import { Route, BrowserRouter as Router, Switch, Redirect } from 'react-router-dom';
import Header from './Components/Header';
import { defaultAuthContext, AuthContext } from './Helpers/AuthContext';
import AuthedRoute from './Pages/AuthedRoute';
import { LandingPage, HomePage, RegisterPage } from './Pages/Pages';

export default function AppRouter() {
    const authContext = useContext(AuthContext);

    useEffect(() => {
        let token = window.localStorage.getItem('token');
        let tempContext = { ...defaultAuthContext };

        if (token !== null) {
            tempContext.authed = true;
            tempContext.token = token;
        }

        tempContext.loading = false;
        
        authContext?.setAuthContext(tempContext);
    }, []);

    return !authContext?.getAuthContext().loading ? (
        <Router>
            <Header />
            <Switch>
                <Route exact path='/'
                    render={() => {
                        return (
                            authContext?.getAuthContext().authed ?
                                <Redirect to="/home" /> :
                                <Redirect to="/welcome" />
                        )
                    }}
                />
                <Route exact path='/welcome'>
                    <LandingPage />
                </Route>
                <Route exact path='/register'>
                    <RegisterPage />
                </Route>

                <AuthedRoute exact path='/home'>
                    <HomePage />
                </AuthedRoute>
            </Switch>
        </Router>
    ) : <></>;
}
