import React, {useState} from 'react';
import Input from "../Components/Input";
import Register from "../Components/Register";

interface IRegisterPageProps {
    
}

export default function RegisterPage(props: IRegisterPageProps) {
    const [email, setEmail] = useState("");
    const [pass, setPass] = useState("");
    const [confirmPass, setConfirmPass] = useState("");
    const [err, setErr] = useState("");

    return (
        <div className='page-container'>
            <h2>Register</h2>
            <Register />
        </div>
    )
}