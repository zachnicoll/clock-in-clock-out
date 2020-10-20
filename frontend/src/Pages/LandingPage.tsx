import React from 'react'
import { useHistory } from 'react-router-dom'

export default function LandingPage({navigation}:any) {
    const history = useHistory();

    return (
        <div className="page-container">
            <h1 style={{fontWeight:"normal", fontSize:"64px", textAlign: "center"}}>Clock-in/Clock-out</h1>
            <hr/>
            <h2>Track every task and ticket in one place.</h2>
            <p>Please log-in to continue!</p>
            <div className="flex-col" style={{width: "50%"}}>
                <button onClick={() => history.push("/login")}>Login</button>
                <button>Register</button>
            </div>
        </div>
    )
}
