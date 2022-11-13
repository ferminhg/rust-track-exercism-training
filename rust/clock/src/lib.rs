use std::fmt;

const MAX_HOURS: i32 = 24;
const MAX_MINUTES: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours_calculated , minutes_calculated) =
            calculate_clock(hours ,minutes);
        Self { hours: hours_calculated, minutes: minutes_calculated }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours_calculated , minutes_calculated) =
            calculate_clock(self.hours, self.minutes + minutes);
        Self { hours: hours_calculated, minutes: minutes_calculated }
    }
}

fn calculate_clock(hours: i32, minutes: i32) -> (i32, i32) {
    let (format_minutes, add_hours) = calculate_minutes(minutes);
    let format_hours = calculate_hours(hours + add_hours);
    (format_hours, format_minutes)
}

fn calculate_hours(hours: i32) -> i32 {
    let format_hours = hours % MAX_HOURS;
    if format_hours < 0 {
        return format_hours + MAX_HOURS;
    }
    format_hours

}

fn calculate_minutes(minutes: i32) -> (i32, i32) {
    if minutes >= 0 {
       return (minutes % MAX_MINUTES, minutes/MAX_MINUTES);
    }

    let mut format_minutes= minutes ;
    let mut hours_added = 0;

    while format_minutes < 0 {
        format_minutes += MAX_MINUTES;
        hours_added -= 1;
    }
    (format_minutes, hours_added)
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}