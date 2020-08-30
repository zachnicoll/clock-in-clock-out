import React, { useState, useEffect } from "react";
import { useTimer } from "../Hooks/useTimer";
import { FormatDuration } from "../Functions/FormatDuration";
import If from "./If";
import moment from "moment";

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

            <div style={{width:"100%", flexWrap: "wrap"}}
            >
                <If condition={!clockedIn}>
                    <img
                        src={require("../Assests/clock_in_un_pressed.svg")}
                        onClick={() => clockIn()}
                        style={{cursor: "pointer"}}
                        alt="Clock In"
                    />

                    <img
                        src={require("../Assests/clock_out_pressed.svg")}
                        alt="Clock Out"
                    />
                </If>

                <If condition={clockedIn}>
                    <img
                        src={require("../Assests/clock_in_pressed.svg")}
                        alt="Clock In"
                    />

                    <img
                        src={require("../Assests/clock_out_un_pressed.svg")}
                        onClick={() => clockOut()}
                        style={{cursor: "pointer"}}
                        alt="Clock Out"
                    />
                </If>
            </div>
        </div>
    );
}
