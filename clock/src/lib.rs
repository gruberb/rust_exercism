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

fn modulus(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m = modulus(minutes, 60);
        let carry_hours = (minutes - m) / 60;

        let h = modulus(hours + carry_hours, 24);

        Clock { hours: h, minutes: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
