
use crate::*;

fn simulation_results_to_buffs(sim_results: &[Option<Buff>]) -> Vec<Buff> {
    sim_results.iter().filter_map(Option::clone).collect()
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
        let result = simulate_once();
        let result = simulation_results_to_buffs(&result);
        tally[result.len() - 1] += 1;
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
pub fn simulation_want_specific() {
    let want: HashSet<Buff> = [Buff::Attack, Buff::ChargeSpeed].into_iter().collect();
    let attempts = 100000;

    let mut hits = 0;
    for _ in 0..attempts {
        let result = simulate_once();
        let result = simulation_results_to_buffs(&result);
        let result: Vec<&Buff> = result
            .into_iter()
            .filter_map(|item| want.get(&item))
            .collect();
        if result.len() == want.len() {
            hits += 1;
        }
    }
    println!(
        "Out of {attempts}, {hits} containted the wanted buffs. That is {}%",
        hits as f64 / attempts as f64 * 100.0
    );
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
        "To get all {:?} for {attempts} times, it required {} custom mods. That is on average {} mods per success.",
        want, sum_custom_mods, 
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
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::Elemental, Buff::MaxAmmo]);
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        sim.set_buff(0, &Buff::Attack);
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

