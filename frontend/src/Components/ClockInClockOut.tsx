import React, { useState, useEffect } from "react";
import { useTimer } from "../Hooks/useTimer";
import { FormatDuration } from "../Functions/FormatDuration";

const clock_in_un_pressed = require("../Assests/clock_in_un_pressed.svg");
const clock_in_pressed = require("../Assests/clock_in_pressed.svg");
const clock_out_un_pressed = require("../Assests/clock_out_un_pressed.svg");
const clock_out_pressed = require("../Assests/clock_out_pressed.svg");

export default function ClockInClockOut() {
    const { getDuration, start, stop } = useTimer();
    const [clockedIn, setClockedIn] = useState(false);

    const clockIn = () => {
        setClockedIn(true);
        start();
    };

    const clockOut = () => {
        setClockedIn(false);
        stop();
    };

    return (
        <div className="clockinoutcontainer">
            <h1 className="time">{FormatDuration(getDuration())}</h1>

            <div style={{width:"100%", flexWrap: "wrap"}}>
                <img
                    src={clockedIn ? clock_in_pressed : clock_in_un_pressed}
                    onClick={() => !clockedIn ? clockIn() : {}}
                    style={{cursor: !clockedIn ? "pointer" : "default"}}
                    alt="Clock In"
                />

                <img
                    src={clockedIn ? clock_out_un_pressed : clock_out_pressed}
                    onClick={() => clockedIn ? clockOut() : {}}
                    style={{cursor: clockedIn ? "pointer" : "default"}}
                    alt="Clock Out"
                />
            </div>
        </div>
    );
}
