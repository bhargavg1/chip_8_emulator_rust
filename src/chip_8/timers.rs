
//! This module is for the chip 8 timers.
//! It contains implementations for the delay timer and sound timer, and the Timable trait for timing the timer ticking.

///This is a delay timer, which will tick down from whatever number it was set to until it reaches 0.
pub struct DelayTimer {
    pub time_value: usize
}

impl DelayTimer {
    ///returns a new DelayTimer object with the time set to 0.
    pub fn new() -> Self {
	return DelayTimer {
	    time_value: 0
	};
    }

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
pub struct SoundTimer {
    pub time_value: usize,
    beep_switch: bool,
    driver: Box<dyn SoundDriver>
}

impl SoundTimer {
    ///Returns a new SoundTimer, with initial time set to 0.
    ///You need to supply a SoundDriver for the timer to use to make it's beeps.
    pub fn new(driver: Box<dyn SoundDriver>) -> Self {
	return SoundTimer {
	    time_value: 0,
	    beep_switch: false,
	    driver
	};
    }

    pub fn tick_down(&mut self) {
	if !self.beep_switch && self.time_value > 0 {
	    self.beep_switch = true;
	    self.driver.set_beep(true);
	    self.time_value -= 1;
	} else if self.beep_switch && self.time_value == 0 {
	    self.beep_switch = false;
	    self.driver.set_beep(false);
	}
    }
}
