import React from 'react';
import { TimeTracker } from '../Components/TimeTracker';
import moment from 'moment';
import ClockInClockOut from '../Components/ClockInClockOut';

export default function HomePage(){

    return(
        <div className="page-container">
            <ClockInClockOut />
            <TimeTracker />
        </div>
    )
} 