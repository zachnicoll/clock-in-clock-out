import moment, { Duration } from "moment";

export function FormatDuration(duration: Duration){
    return `${duration.hours()}h ${duration.minutes()}m ${duration.seconds()}s`;
}