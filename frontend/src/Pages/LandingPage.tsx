import React from 'react'
import { useHistory } from 'react-router-dom'
import Login from '../Components/Login';

export default function LandingPage({ navigation }: any) {
    const history = useHistory();

    return (
        <div className="page-container h-space">
            <h1 style={{ fontWeight: "normal", fontSize: "48px", textAlign: "center" }}>Clock-in/Clock-out</h1>
            <hr />
            <h2 style={{ textAlign: "center" }}>Track every task and ticket in one place.</h2>

            <Login />

            <p>Don't have an account? <a href="/register">Register.</a></p>
        </div>
    )
}
