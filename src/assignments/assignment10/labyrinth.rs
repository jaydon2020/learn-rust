//! Labyrinth
//!
//! Look at `labyrinth_grade.rs` below before you start.
//! HINT: <https://en.wikipedia.org/wiki/100_prisoners_problem>
//!
//! NOTE: You will have to implement a probabilistic algorithm, which means, the algorithm can fail
//! even if you have implemented the solution. We recommend running multiple times (at least 5
//! times) to check your solution works well.

use std::cell::RefCell;

/// Husband
#[derive(Debug)]
pub struct Husband {
    brain: RefCell<[usize; 100]>,
}

impl Husband {
    /// What might a husband, who is looking for his wife's ID my_wife, be thinking?
    pub fn seeking(my_wife: usize) -> Self {
        // 1) Initialize the brain
        let brain = RefCell::new([0usize; 100]);

        // 2) Mutably borrow the inner array and write the ID
        {
            let mut memory = brain.borrow_mut();
            memory[0] = my_wife; // store at index 0

            // Alternatively, to distribute by ID:
            // let idx = my_wife % memory.len();
            // memory[idx] = my_wife;
        } // <- drop the borrow before returning

        Husband { brain }
    }

    #[allow(missing_docs)]
    pub fn has_devised_a_strategy(&self) -> Strategy<'_> {
        Strategy {
            husband: self,
            last_room: None,
            remaining: 50, // typical limit in the 100 prisoners problem
        }
    }

    /// Based on the information about currently visited room number and someone's wife ID trapped
    /// inside, what the husband should do next?
    pub fn carefully_checks_whos_inside(&self, room: usize, wife: usize) {
        let mut mem = self.brain.borrow_mut();

        let idx = room % mem.len();
        mem[idx] = wife;
    }
}

/// Strategy of husband
#[derive(Debug)]
pub struct Strategy<'a> {
    husband: &'a Husband,
    /// The most recently yielded room (used to look up the next room from the brain).
    last_room: Option<usize>,
    /// How many steps we have left. Default is 50 for the classic setting.
    remaining: usize,
}

impl Iterator for Strategy<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        // Decide the next room:
        // - On the first step, go to the seed at brain[0] (set in `seeking`)
        // - Afterwards, go to brain[last_room], which was filled by `carefully_checks_whos_inside`
        let room_to_visit = {
            let mem = self.husband.brain.borrow();
            match self.last_room {
                None => mem[0] % mem.len(),                    // first step uses the seed
                Some(prev_room) => mem[prev_room % 100] % 100, // follow the permutation
            }
        };

        self.last_room = Some(room_to_visit);
        self.remaining -= 1;
        Some(room_to_visit)
    }
}
