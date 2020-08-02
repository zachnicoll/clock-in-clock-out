import { Duration } from "moment";

export function FormatDuration(duration: Duration){
    let seconds = Math.round(duration.asSeconds());
    let minutes = 0;
    let hours = 0;

    if (seconds / 60 >= 1) {
      minutes = seconds / 60;
    }

    if (seconds / 60 / 60 >= 1) {
      hours = seconds / 60 / 60;
    }

    if (minutes >= 1) {
      seconds = Math.round(60 * (minutes - Math.floor(minutes)));
    }

    return `${Math.floor(hours)}h ${Math.floor(minutes)}m ${seconds}s`;
}