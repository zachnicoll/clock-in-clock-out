import React, { useState, useEffect } from "react";
import moment, { Duration } from "moment";

export function useTimer() {
    const [startTime, setStartTime] = useState(moment()); // Moment the timer started
    const [timing, setTiming] = useState(false); // Is the Timer timing?
    const [durationArray, setDurationArray] = useState<Duration[]>([]); // Array of durations for tracking time, helps with stopping/starting

    const start = () => {
        setStartTime(moment());
        setDurationArray([...durationArray, moment.duration(0)]);
        setTiming(true);
    };

    const stop = () => {
        setTiming(false);
    };

    const reset = () => {
        stop();
        setDurationArray([]);
    };

    const getDuration = () => {
        const durationSum = durationArray.reduce((a, b) => {
            return a.add(b);
        }, moment.duration(0));

        return durationSum;
    };

    function currentElapsed() {
        if (timing) {
            let temp = [...durationArray];
            temp[temp.length - 1] = moment.duration(moment().diff(startTime));
            setDurationArray(temp);
        }
    }

    useEffect(() => {
        let timer = setTimeout(() => {}, 0); // Need to do this because TypeScript

        if (timing) {
            timer = setTimeout(() => currentElapsed(), 20);
        }

        return () => {
            clearTimeout(timer);
        };
    }, [timing, durationArray]);

    return { getDuration, start, stop, reset, timing };
}
