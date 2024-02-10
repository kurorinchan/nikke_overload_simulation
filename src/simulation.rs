
use itertools::join;

use crate::*;

const DEFAULT_ATTEMPTS : u32 = 100000;

fn buffs_to_string<'a, I>(buffs: I) -> String 
where
I: Iterator<Item = &'a Buff>
{
    buffs.map(|item| {
        format!(
            "{:?}({}%)", item, item.percent())
    }).collect::<Vec<String>>().join(" ")
}

// This is more of a check/test than a simluation.
// This verifies that the distribution of the slots being populated matches the expected.
// Only one (which is the first slot)  populated 35%.
// Any two slots populated is 50%.
// All two slots populated is 15%.
pub fn simulation_slots_shown_distribution() {
    let attempts = 100000;

    let mut tally = [0, 0, 0];
    for _ in 0..attempts {
        let mut sim = Simulation::new();
        sim.reroll();
        let num_buffs = sim.buffs().iter().filter_map(|item| {
            match item {
                SlotState::Free(b) | SlotState::Locked(b) => Some(b),
                _ => None,
            }
        }).count();
        tally[num_buffs - 1] += 1;
    }

    println!("Tally of buff nums! {:?}", tally);
    let sum: usize = tally.iter().sum();

    let one = tally[0] as f64 / sum as f64 * 100.0;
    let two = tally[1] as f64 / sum as f64 * 100.0;
    let three = tally[2] as f64 / sum as f64 * 100.0;

    println!("Percentage of buffs {:.2}% {:.2}% {:.2}%", one, two, three);
}

// The number of attempts required to get all the wanted buffs, without locking.
// For example if you want Attack and ChargeSpeed. This simulation checks how many rolls
// got you all the buffs that you want. Divide that number by the total attempts should get us
// the probability.
fn sim_want_two_buffs(want: &[Buff]) {
    let want : HashSet<Buff> = HashSet::from_iter(want.iter().copied());
    let attempts = DEFAULT_ATTEMPTS;

    let mut hits = 0;
    for _ in 0..attempts {
        let mut sim = Simulation::new();
        sim.reroll();
        let mut found = HashSet::new();
        for b in want.iter() {
            if sim.has_buff(b) {
                found.insert(*b);
            }
        }

        if found.eq(&want) {
            hits += 1;
        }
    }

    println!(
        "To get buffs {}. \
        The simulation ran {attempts} rerolls, which {hits} had \
        all the buffs. This is about {:.2}%.",
        buffs_to_string(want.iter())
        , hits as f64 / attempts as f64 * 100.0);
}

// A suite of simluations.
// There are two types of buffs, those that appear 10% of the time and those that appear 12%.
// This suite runs a few simluations.
// Simulation that aims for 10% and 10% buffs.
// Simulation that aims for 10% and 12% buffs.
// Simulation that aims for 12% and 12% buffs.

pub fn suite_two_desired_buffs() {
    println!("Start suite: two desired buffs.");
    sim_want_two_buffs(&[Buff::Attack, Buff::Elemental]);
    sim_want_two_buffs(&[Buff::Attack, Buff::MaxAmmo]);
    sim_want_two_buffs(&[Buff::ChargeDamage, Buff::ChargeSpeed]);
    println!("End suite: two desired buffs.");
}

fn reroll_until_all_found(sim: &mut Simulation, want: &HashSet<Buff>) {
    loop {
        sim.reroll();
        let mut found = HashSet::new();

        for b in want.iter() {
            if sim.has_buff(b) {
                found.insert(b.clone());
            }
        }

        if want.eq(&found) {
            break;
        }
    }
}

fn reroll_until_all_found_with_locking(sim: &mut Simulation, want: &HashSet<Buff>) {
    loop {
        sim.reroll();
        let mut found = HashSet::new();

        for b in want.iter() {
            if sim.has_buff(b) {
                let pos = sim.position_of(b).unwrap();
                sim.lock(pos);
                found.insert(b.clone());
            }
        }

        if want.eq(&found) {
            break;
        }
    }
}

// Simulation to see how many custom modules are required to get all buffs without locking.
// For example if you want Attack and ChargeSpeed. This simulation checks how many custom modules
// are used to got you all the buffs that you want.
// This runs the process multiple times and gets the average number of modules required, without
// locking.
// Oddly this does not match with the simulation that gets the probability of getting all buffs.
pub fn simulation_num_cus_mod_for_specific() {
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::MaxAmmo]);
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {} for {attempts} times, it required {} custom mods. That is on average {} mods per success.",
        buffs_to_string(want.iter()), sum_custom_mods, 
        sum_custom_mods as f64 / attempts as f64
    );
}

// Simulate to see how many custom modules are required to get a specific set of buffs, with locking.
pub fn simulation_num_cus_mods_with_locking() {
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::MaxAmmo]);
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found_with_locking(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {:?} for {attempts} times, it required {} custom mods. That is on average {} mods per success.",
        want, sum_custom_mods, 
        sum_custom_mods as f64 / attempts as f64
    );
}

// Find custom module usage given that a desired buff is locked on the firsr slot.
pub fn simulation_first_desired_buff_locked() {
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::MaxAmmo]);
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        sim.set_buff(0, &Buff::MaxAmmo);
        sim.lock_first();

        reroll_until_all_found_with_locking(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {:?} for {attempts} times, it required {} custom mods. That is on average {} mods per success.",
        want, sum_custom_mods, 
        sum_custom_mods as f64 / attempts as f64
    );
}


// Find custom module usage given that a desired buff is locked on the second slot.
pub fn simulation_second_desired_buff_locked() {
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::MaxAmmo]);
    let attempts = 100000;

    let mut sum_custom_mods = 0;
    let mut sum_rerolls = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        sim.set_buff(1, &Buff::MaxAmmo);
        sim.lock_second();

        reroll_until_all_found_with_locking(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
        sum_rerolls += sim.attempts;
    }

    println!(
        "To get all {:?} for {attempts} times, it required {} custom mods and {} rerolls.\n\
        That is on average {} mods per success and {} rerolls.",
        want, sum_custom_mods, sum_rerolls,
        sum_custom_mods as f64 / attempts as f64,
        sum_rerolls as f64/attempts as f64,
    );
}