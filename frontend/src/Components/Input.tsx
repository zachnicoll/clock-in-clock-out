import React, { useState, ChangeEvent, useEffect } from "react";

interface IInputProps {
    name: string;
    placeholder?: string;
    onChange: (text: string) => void;
    label?: string;
    className?: string;
    type?: string;
    value?: string;
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
        <div className="input-container full-width">
            <p>{props.label}</p>
            <input
                name={props.name}
                type={props.type ? props.type : "text"}
                className={props.className}
                placeholder={props.placeholder}
                value={text}
                onChange={onChange}
            />
        </div>
    );
}
