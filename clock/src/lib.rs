use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h: i32 = hours;
        
        let mut m: i32 = minutes;
        if m >= 60 {
            h = h + m / 60;
            m = m % 60;
        }

        while m < 0 {
            m = 60 + m;
            h = h - 1;
        }

        if h >= 24 {
            h = h % 24;
        }

        while h < 0 {
            h = 24 + h;
        }

        Clock {
            hours: h,
            minutes: m
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}