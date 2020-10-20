import React, { useState, useEffect } from "react";
import moment, { Moment, Duration } from "moment";
import { AccessTime, PlayArrow, Stop } from "@material-ui/icons";
import { FormatDuration } from "../Functions/FormatDuration";
import { useTimer } from "../Hooks/useTimer";
import If from "./If";
import TrackerCard, { ITrackerCardProps } from "./TrackerCard";
import Input from "./Input";
import ComboBox from "./ComboBox";

export function TimeTracker() {
    const { getDuration, start, stop, reset, timing } = useTimer();
    const [tasks, setTasks] = useState<ITrackerCardProps[]>([]);
    const [ticket, setTicket] = useState("");
    const [tag, setTag] = useState("");
    const [label, setLabel] = useState("");

    function AddTask() {
        const newTask: ITrackerCardProps = {
            label: label,
            ticket: ticket,
            date: moment(),
            tag: tag,
            duration: getDuration(),
        };

        setTasks([...tasks, newTask]);

        setLabel("");
        setTicket("");

        reset();
    }

    return (
        <div style={{ width: "100%" }}>
            <div className="tracker-band">
                    <Input
                        name="ticket"
                        type="text"
                        className="trackerinput"
                        placeholder="[UNISD-101]"
                        label="Ticket"
                        value={ticket}
                        onChange={setTicket}
                    />

                    <ComboBox
                        name="tag"
                        label="Tag"
                        className="trackerinput"
                        onChange={setTag}
                        value={tag}
                        options={["Project Work", "Meeting", "Break", "Other"]}
                    />

                    <Input
                        name="label"
                        type="text"
                        className="trackerinput"
                        placeholder="Fixing issue..."
                        label="Label"
                        value={label}
                        onChange={setLabel}
                    />
                    <div className="flex-row">
                        <h2>{FormatDuration(getDuration())}</h2>

                        <If condition={!timing}>
                            <div onClick={() => start()}>
                                <PlayArrow className="trackericon" />
                            </div>
                        </If>

                        <If condition={timing}>
                            <div onClick={() => AddTask()}>
                                <Stop className="trackericon" />
                            </div>
                        </If>
                    </div>
            </div>

            <div className="cardcontainer">
                {tasks.map((task: ITrackerCardProps) => {
                    return <TrackerCard {...task} />;
                })}
            </div>
        </div>
    );
}
