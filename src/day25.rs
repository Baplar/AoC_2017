//! This day is a bit different,
//! since the parsing phase is difficult to implement
//! but fast to do manually.
//! Hence, this problem will be solved directly.

use std::collections::{HashMap, HashSet};
use self::Dir::{Left, Right};

enum Dir {
    Left,
    Right,
}

struct Rule {
    value: bool,
    direction: Dir,
    next: char,
}

impl Rule {
    fn new(value: bool, direction: Dir, next: char) -> Rule {
        Rule {
            value,
            direction,
            next,
        }
    }
}

struct State {
    zero: Rule,
    one: Rule,
}

impl State {
    fn from_rules(zero: Rule, one: Rule) -> State {
        State { zero, one }
    }

    fn from_params(
        value_zero: bool,
        direction_zero: Dir,
        next_zero: char,
        value_one: bool,
        direction_one: Dir,
        next_one: char,
    ) -> State {
        State::from_rules(
            Rule::new(value_zero, direction_zero, next_zero),
            Rule::new(value_one, direction_one, next_one),
        )
    }
}

#[derive(Default)]
struct Blueprint {
    init: char,
    states: HashMap<char, State>,
    duration: usize,
}

impl Blueprint {
    fn my_blueprint() -> Blueprint {
        let init = 'A';
        let duration = 12_425_180;
        let mut states = HashMap::new();

        states.insert('A', State::from_params(true, Right, 'B', false, Right, 'F'));

        states.insert('B', State::from_params(false, Left, 'B', true, Left, 'C'));

        states.insert('C', State::from_params(true, Left, 'D', false, Right, 'C'));

        states.insert('D', State::from_params(true, Left, 'E', true, Right, 'A'));

        states.insert('E', State::from_params(true, Left, 'F', false, Left, 'D'));

        states.insert('F', State::from_params(true, Right, 'A', false, Left, 'E'));

        Blueprint {
            init,
            states,
            duration,
        }
    }
}

struct Turing {
    step: usize,
    state: char,
    tape: HashSet<isize>,
    cursor: isize,
    blueprint: Blueprint,
}

impl Turing {
    fn new(blueprint: Blueprint) -> Self {
        Turing {
            step: 0,
            state: blueprint.init,
            tape: HashSet::new(),
            cursor: 0,
            blueprint,
        }
    }

    fn next(&mut self) -> (usize, char, isize, bool) {
        self.step += 1;

        let state = self.blueprint
            .states
            .get(&self.state)
            .expect(&format!("No rule for state {}", self.state));

        let rule = if self.tape.contains(&self.cursor) {
            &state.one
        } else {
            &state.zero
        };

        if rule.value {
            self.tape.insert(self.cursor);
        } else {
            self.tape.remove(&self.cursor);
        }

        match rule.direction {
            Right => self.cursor += 1,
            Left => self.cursor -= 1,
        }

        self.state = rule.next;

        (
            self.step,
            self.state,
            self.cursor,
            self.tape.contains(&self.cursor),
        )
    }
}

pub fn one(_s: &str) -> String {
    let mut turing = Turing::new(Blueprint::my_blueprint());

    for i in 0..turing.blueprint.duration {
        turing.next();
        if i % 1_000_000 == 0 {
            eprintln!("{}", i);
        }
    }

    turing.tape.len().to_string()
}
