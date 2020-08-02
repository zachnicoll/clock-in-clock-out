import React from "react";
import { Settings, Person, BarChart } from "@material-ui/icons";

export default function Header() {
    return (
        <div className="headercontainer">
            <h3 style={{ fontWeight: 500, marginLeft: 15, color: "white" }}>
                Clock-in/Clock-out
            </h3>
            <div style={{ display: "flex", flexDirection: "row-reverse" }}>
                <div className="headerbutton">
                    <Settings className="headericon" />
                </div>
                <div className="headerbutton">
                    <Person className="headericon" />
                </div>
                <div className="headerbutton">
                    <BarChart className="headericon" />
                </div>
            </div>
        </div>
    );
}
