export function convertSecondsToHumanReadable(seconds: number) {
    let mins = seconds/60;
    if (mins >= 60) {
        let hours = mins/60;
        if (hours >= 24) {
            let days = hours/24;
            return `${Math.floor(days)} days`
        }
        return `${Math.floor(hours)} hours`
    }
    return `${Math.floor(mins)} mins`
}

export function isStringBlank(str: string | null | undefined) {
    return str == null || str == undefined || str.trim().length == 0
}

export function convertEpochToIsoFormat(seconds: number) {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;

    return `${String(h).padStart(2, '0')}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}