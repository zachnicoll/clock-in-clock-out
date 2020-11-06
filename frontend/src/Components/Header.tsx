import React from "react";
import { Settings, Person, BarChart } from "@material-ui/icons";

export default function Header() {
    return (
        <div className="header-container">
            <a href={'/home'}>
                <h3 style={{ fontWeight: 500, marginLeft: 15, color: "white" }}>
                    Clock-in/Clock-out
                </h3>
            </a>
            <div style={{ display: "flex", flexDirection: "row-reverse" }}>
                <div className="header-button">
                    <Settings className="header-icon" />
                </div>
                <div className="header-button">
                    <Person className="header-icon" />
                </div>
                <div className="header-button">
                    <BarChart className="header-icon" />
                </div>
            </div>
        </div>
    );
}
