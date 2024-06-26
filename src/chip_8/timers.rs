
//! This module is for the chip 8 timers.
//! It contains implementations for the delay timer and sound timer.

///This is a delay timer, which will tick down from whatever number it was set to until it reaches 0.
pub struct DelayTimer {
    pub time_value: u8
}

impl DelayTimer {
    ///returns a new DelayTimer object with the time set to 0.
    pub fn new() -> Self {
	return DelayTimer {
	    time_value: 0
	};
    }

    ///decrements the timer once everytime it is called unless the timer has already reached 0.
    pub fn tick_down(&mut self) {
	if self.time_value > 0 {
	    self.time_value -= 1;
	}
    }
}

///This defines how a sound driver should behave so that the chip 8 can use it.
///multiple sound drivers can be used so that the chip 8 can beep in different ways.
pub trait SoundDriver {
    fn set_beep(&self, state: bool);
}

///This is the sound timer, which will continuously beep as long as it is above 0. It ticks down until it reaches 0.
pub struct SoundTimer <'a> {
    pub time_value: u8,
    driver: Box<dyn SoundDriver + 'a>
}

impl <'a> SoundTimer <'a> {
    ///Returns a new SoundTimer, with initial time set to 0.
    ///You need to supply a SoundDriver for the timer to use to make it's beeps.
    pub fn new<T: SoundDriver + 'a>(driver: T) -> Self {
	return SoundTimer {
	    time_value: 0,
	    driver: Box::new(driver)
	};
    }

    ///decrements the timer once everytime it is called, unless the timer has already reached 0.
    ///This will also cause constant beeping to happen as long as the timer is above 0.
    pub fn tick_down(&mut self) {
	if self.time_value > 0 {
	    self.time_value -= 1;
	    self.driver.set_beep(true);
	} else {
	    self.driver.set_beep(false);
	}
    }
}
