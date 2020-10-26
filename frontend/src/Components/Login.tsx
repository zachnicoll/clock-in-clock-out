import Axios from 'axios';
import React, { ChangeEvent, useState } from 'react'
import Input from './Input';

export default function Login() {
    const [email, setEmail] = useState("");
    const [pass, setPass] = useState("");

    function login()
    {
        Axios.post('/api/users/login', {
            email: email,
            password: pass
        })
        .then(res => console.log(res))
        .catch(console.warn);
    }

    return (
        <div className="flex-col full-width">
            <Input 
                placeholder="Email"
                onChange={setEmail}
                name="Email"
                type="email"
                label="Email"
                className="login-input full-width"
            />
            <Input 
                placeholder="Password"
                onChange={setEmail}
                name="Password"
                type="password"
                label="Password"
                className="login-input full-width"
            />
            <button onClick={() => login()}>Login</button>
        </div>
    )
}
