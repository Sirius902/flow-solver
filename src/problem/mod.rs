use std::hash::Hash;

pub mod flow;

pub type Cost = ordered_float::NotNan<f32>;

// Safe because we know 0.0 isn't NaN.
pub const ZERO_COST: Cost = unsafe { Cost::new_unchecked(0.0) };

pub trait Problem {
    type State: Eq + Hash + Clone;
    type Action: Clone;
    type SuccessorIter: Iterator<Item = (Self::State, Self::Action, Cost)>;

    fn starting_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
    fn successor_states(&self, state: &Self::State) -> Self::SuccessorIter;
}
