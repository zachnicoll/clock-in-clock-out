import Axios from 'axios';
import React, { useContext, useState } from 'react'
import { useHistory } from 'react-router-dom';
import { AuthContext } from '../Helpers/AuthContext';
import Input from './Input';

export default function Login() {
    const [email, setEmail] = useState("");
    const [pass, setPass] = useState("");
    const [err, setErr] = useState("");
    const history = useHistory();
    const setAuthContext = useContext(AuthContext)?.setAuthContext;

    function login()
    {
        Axios.post('/api/users/login', {
            email: email,
            password: pass
        })
        .then(res => {
            setAuthContext ? setAuthContext({
                authed: true,
                token: res.data.token,
                userId: res.data.user.id,
                loading: false
            }) : console.error("AUTH CONTEXT IS UNDEFINED!!!");
            window.localStorage.setItem('token', res.data.token);
            history.push('/home');
        })
        .catch(e => setErr("Email or password incorrect."));
    }

    return (
        <div className="flex-col half-width login-container">
            <Input 
                placeholder="Email"
                onChange={setEmail}
                name="Email"
                type="email"
                label="Email"
                className="login-input full-width"
                containerClassName="full-width"
                value={email}
            />
            <Input 
                placeholder="Password"
                onChange={setPass}
                name="Password"
                type="password"
                label="Password"
                className="login-input full-width"
                containerClassName="full-width"
                value={pass}
            />
            <p style={{color:"red"}}>{err}</p>
            <button onClick={() => login()}>Login</button>
        </div>
    )
}
