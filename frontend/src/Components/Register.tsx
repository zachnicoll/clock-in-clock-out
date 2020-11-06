import React, {useState} from 'react';
import Input from "./Input";
import Axios from "axios";
import {ILoginData} from "../Models/ILoginData";

interface IRegisterProps {

}

const requiredFieldErr = "Field must not be empty";
const passwordTooShortErr = "Password must be at least 8 characters long";
const passwordMustMatchErr = "Passwords must match";

export default function Register(props: IRegisterProps) {
    const [email, setEmail] = useState("");
    const [pass, setPass] = useState("");
    const [confirmPass, setConfirmPass] = useState("");
    const [err, setErr] = useState<string[]>([]);

    const register = async () => {
        let tempErr: string[] = [];

        if (email.length === 0) {
            tempErr[0] = requiredFieldErr;
        }

        if (pass.length < 8) {
            tempErr[1] = passwordTooShortErr;
        }

        if (pass !== confirmPass) {
            tempErr[2] = passwordMustMatchErr;
        }

        if (tempErr.length === 0) {
            let newUser:ILoginData = {
                email: email,
                password: pass
            }

            await Axios.post('/api/users/', newUser)
                .then(_ => window.location.href = '/welcome')
                .catch(e => {
                    tempErr[3] = e.response.data.message;
                })
        }

        setErr(tempErr);
    }

    return (
        <div className='flex-col half-width login-container'>
            <Input
                placeholder="Email"
                onChange={setEmail}
                name="Email"
                type="email"
                label="Email"
                className="login-input full-width"
                containerClassName="full-width"
                value={email}
                errorMessage={err[0]}
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
                errorMessage={err[1]}
            />
            <Input
                placeholder="Confirm password"
                onChange={setConfirmPass}
                name="Confirm Password"
                type="password"
                label="Confirm Password"
                className="login-input full-width"
                containerClassName="full-width"
                value={confirmPass}
                errorMessage={err[2]}
            />
            <p style={{color:"red"}}>{err[3]}</p>
            <button onClick={() => register()}>Register</button>
        </div>
    )
}