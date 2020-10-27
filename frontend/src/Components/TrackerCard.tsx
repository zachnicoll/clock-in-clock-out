import React, { useState, useEffect } from "react";
import { FormatDuration } from "../Functions/FormatDuration";
import { Duration, Moment } from "moment";
import ComboBox from "./ComboBox";
import Input from "./Input";

export interface ITrackerCardProps {
    ticket: string;
    tag: string;
    label: string;
    duration: Duration;
    date: Moment;
}

export default function TrackerCard(props: ITrackerCardProps) {
    const [ticket, setTicket] = useState(props.ticket);
    const [tag, setTag] = useState(props.tag);
    const [label, setLabel] = useState(props.label);
    const [duration, setDuration] = useState<Duration>(props.duration);
    const [date, setDate] = useState(props.date);

    return (
        <div className="tracker-card flex-row">
                <Input
                    name="ticket"
                    type="text"
                    className="tracker-input"
                    label="Ticket"
                    onChange={setTicket}
                    value={props.ticket}
                />

                <ComboBox
                    name="tag"
                    label="Tag"
                    className="tracker-input"
                    onChange={setTag}
                    options={["Project Work", "Meeting", "Break", "Other"]}
                    value={props.tag}
                />

                <Input
                    name="label"
                    type="text"
                    className="tracker-input"
                    label="Label"
                    onChange={setLabel}
                    value={props.label}
                />
            <h2>{FormatDuration(duration)}</h2>
        </div>
    );
}
