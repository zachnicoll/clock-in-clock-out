import React, { useState, ChangeEvent, useEffect } from "react";

interface IInputProps {
    name: string;
    placeholder?: string;
    onChange: (text: string) => void;
    label?: string;
    className?: string;
    containerClassName?: string;
    type?: string;
    value?: string;
    errorMessage?: string;
}

export default function Input(props: IInputProps) {
    const [text, setText] = useState(props.value ? props.value : "");

    useEffect(() => {
        props.onChange(text);
    }, [text])

    useEffect(() => {
        if(props.value === ""){
            setText("");
        }
    }, [props.value])

    function onChange(e: any){
        setText(e.target.value);
    }

    return (
        <div className={`input-container ${props.containerClassName}`}>
            <p>{props.label}</p>
            <input
                name={props.name}
                type={props.type ? props.type : "text"}
                className={props.className}
                placeholder={props.placeholder}
                value={text}
                onChange={onChange}
            />
            {props.errorMessage && <p className='error-msg'>{props.errorMessage}</p>}
        </div>
    );
}
