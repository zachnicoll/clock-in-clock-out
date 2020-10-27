import React, { useState, ChangeEvent, useEffect } from "react";

interface IComboProps {
    name: string;
    options: string[];
    onChange: (text: string) => void;
    label?: string;
    className?: string;
    value?: string;
}

export default function ComboBox(props: IComboProps) {
    const [option, setOption] = useState(props.value ? props.value : "");

    useEffect(() => {
        props.onChange(option);
    }, [option]);

    function onChange(e: any) {
        setOption(e.target.value);
    }

    return (
        <div className="input-container">
            <p>{props.label}</p>
            <select name={props.name} className={props.className} onChange={onChange} value={option}>
                {props.options.map((option: string) => {
                    return <option key={option} value={option}>{option}</option>;
                })}
            </select>
        </div>
    );
}
