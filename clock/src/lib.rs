use std::fmt::{self, Debug, Display};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut c = Self {
            hours: 0,
            minutes: 0,
        };
        c.add_hours(hours);
        c.add_minutes(minutes);
        c
    }

    fn add_hours(&mut self, hours: i32) {
        self.hours += hours;

        self.hours %= 24;

        if self.hours < 0 {
            self.hours += 24;
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.add_hours(minutes / 60);
        self.minutes += minutes % 60;

        if self.minutes > 60 {
            self.minutes %= 60;
            self.add_hours(1);
        }

        if self.minutes < 0 {
            self.minutes += 60;
            self.add_hours(-1);
        }
        *self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
