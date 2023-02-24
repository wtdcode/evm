//! Allows to listen to runtime events.

use crate::Machine;
environmental::environmental!(listener: dyn EventListener + 'static);

pub trait EventListener {
	fn event(&mut self, event: Event);
}

#[derive(Copy, Clone)]
pub enum Event<'a> {
	JUMP {
        machine: &'a Machine,
		dest: usize
	},
    JUMPI {
        machine: &'a Machine,
        dest: usize
    }
}

// Expose `listener::with` to the crate only.
pub(crate) fn with<F: FnOnce(&mut (dyn EventListener + 'static))>(f: F) {
	listener::with(f);
}

/// Run closure with provided listener.
pub fn using<R, F: FnOnce() -> R>(new: &mut (dyn EventListener + 'static), f: F) -> R {
	listener::using(new, f)
}
