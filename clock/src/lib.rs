use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes &&
            self.hours == other.hours
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = minutes + (hours * 60);
        let hours: f32 = hours as f32 / 60.0;
        let minutes = hours.fract() * 60.0;
        let minutes  = minutes.round() as i32;
        let hours: i32 = hours as i32 % 24;

        Clock {
            minutes,
            hours,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
