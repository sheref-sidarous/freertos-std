use crate::cell::Cell;
use crate::sync as public;
use crate::sync::once::ExclusiveState;
use crate::sync::{Mutex, MutexGuard};

pub struct Once {
    state: Mutex<State>,
}

pub struct OnceState {
    poisoned: bool,
    set_state_to: Cell<State>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Incomplete,
    Poisoned,
    Running,
    Complete,
}

struct CompletionGuard<'a> {
    state: MutexGuard<'a, State>,
    set_state_on_drop_to: State,
}

impl<'a> Drop for CompletionGuard<'a> {
    fn drop(&mut self) {
        *self.state = self.set_state_on_drop_to;
    }
}

// Safety: threads are not supported on this platform.
unsafe impl Sync for Once {}

impl Once {
    #[inline]
    #[rustc_const_stable(feature = "const_once_new", since = "1.32.0")]
    pub const fn new() -> Once {
        Once { state: Mutex::new(State::Incomplete) }
    }

    #[inline]
    pub fn is_completed(&self) -> bool {
        let state = self.state.lock().unwrap();
        *state == State::Complete
    }

    #[inline]
    pub(crate) fn state(&self) -> ExclusiveState {
        let state = self.state.lock().unwrap();
        match *state {
            State::Incomplete => ExclusiveState::Incomplete,
            State::Poisoned => ExclusiveState::Poisoned,
            State::Complete => ExclusiveState::Complete,
            _ => unreachable!("invalid Once state"),
        }
    }

    #[cold]
    #[track_caller]
    pub fn call(&self, ignore_poisoning: bool, f: &mut impl FnMut(&public::OnceState)) {
        let mut state = self.state.lock().unwrap();
        let is_poisoned =  *state == State::Poisoned;
        match *state {
            State::Poisoned if !ignore_poisoning => {
                // Panic to propagate the poison.
                panic!("Once instance has previously been poisoned");
            }
            State::Incomplete | State::Poisoned => {
                *state = State::Running;
                // `guard` will set the new state on drop.
                let mut guard =
                    CompletionGuard { state: state, set_state_on_drop_to: State::Poisoned };
                // Run the function, letting it know if we're poisoned or not.
                let f_state = public::OnceState {
                    inner: OnceState {
                        poisoned: is_poisoned,
                        set_state_to: Cell::new(State::Complete),
                    },
                };
                f(&f_state);
                guard.set_state_on_drop_to = f_state.inner.set_state_to.get();
            }
            State::Running => {
                panic!("one-time initialization may not be performed recursively");
            }
            State::Complete => {}
        }
    }
}

impl OnceState {
    #[inline]
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    #[inline]
    pub fn poison(&self) {
        self.set_state_to.set(State::Poisoned)
    }
}
