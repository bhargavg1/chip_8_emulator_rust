
//! This module is for the chip 8 timers.
//! It contains implementations for the delay timer and sound timer, and the Timable trait for timing the timer ticking.

///Depending on the object's internal settings, calling do_act() in your loop will cause the object to perform it's action a fixed number of times per second.
///If you have a for loop that loops and calls do_act() 1000 times per second, and the object's internal settings
/// dictate that it should perform it's action only 10 times per second, do_act() will cause the object to perform it's action only once every 100 loops.
///current_moment: the number of loops youve gone through so far (for, while, loop, etc.).
///second_size: the amount of loops in a second that your loop can do.
pub trait Timable {
    const ACTIONS_PER_SECOND: usize;
    ///when put in a loop, this will do an action a certain amount of times per second.
    fn do_act(&mut self, current_moment: usize, second_size: usize);
 
    ///returns true a set amount of times per second.
    fn now(current_moment: usize, second_size: usize) -> bool {
	return (current_moment % second_size) % (second_size / Self::ACTIONS_PER_SECOND) == 0;
    }
}

///This defines a timer object, which counts down and does an action when the timer is at certain states.
pub struct Timer {
    time: u8,
    action: Box<dyn Fn(usize)>
}

impl Timer {
    ///creates a new timer object with the defined action.
    ///by default, the timer is just at 0.
    fn new(action: Box<dyn Fn(usize)>) -> Self {
	return Timer {
	    time: 0,
	    action
	};
    }
}

impl Timable for Timer {
    const ACTIONS_PER_SECOND: usize = 60;
    
    ///in this implementation, the do_act() function will tick down the timer roughly 60 times per second.
    fn do_act(&mut self, current_moment: usize, second_size: usize) {
	if Self::now(current_moment, second_size) {
	    if self.time > 0 {
		self.time -= 1;
		(*self.action)(self.time.into());
	    }
	}
    }
}

impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	return f.debug_struct("Timer")
	    .field("time", &self.time)
	    .finish();
    }
}

pub fn delay_timer() -> Timer {
    return Timer::new(Box::new(|_x| {}));
}

pub fn sound_timer() -> Timer {
    return Timer::new(Box::new(|_x| {}));
}
