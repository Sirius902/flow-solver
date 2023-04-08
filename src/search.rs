use std::collections::{BinaryHeap, HashMap};

use crate::problem::{Cost, Problem, ZERO_COST};

pub fn a_star_search<P: Problem>(
    problem: &P,
    heuristic: impl Fn(&P, &P::State) -> Cost,
) -> Option<Vec<P::Action>> {
    let start = problem.starting_state();

    let mut fringe = BinaryHeap::new();
    fringe.push(FringeNode(ZERO_COST, start.clone()));

    let mut paths = HashMap::new();
    paths.insert(start, (ZERO_COST, Vec::new()));

    while let Some(FringeNode(_, state)) = fringe.pop() {
        if problem.is_goal(&state) {
            return Some(paths.remove(&state).unwrap().1);
        }

        for (succ, action, succ_cost) in problem.successor_states(&state) {
            let (path_cost, path) = paths.get(&state).unwrap();

            let mut succ_path = path.clone();
            succ_path.push(action);

            let h = heuristic(problem, &succ);
            paths.insert(succ, (path_cost + succ_cost + h, succ_path));
        }
    }

    None
}

struct FringeNode<S>(Cost, S);

impl<S> PartialEq for FringeNode<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S> Eq for FringeNode<S> {}

impl<S> PartialOrd for FringeNode<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<S> Ord for FringeNode<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn null_heuristic<P: Problem>(_: &P, _: &P::State) -> Cost {
    ZERO_COST
}
