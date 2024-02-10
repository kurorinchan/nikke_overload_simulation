use itertools::join;

use crate::*;

const DEFAULT_ATTEMPTS: u32 = 100000;

const START_SUITE_MARKER: &'static str = "===== SUITE START =====";
const END_SUITE_MARKER: &'static str = "=====  SUITE END  =====";

fn buffs_to_string<'a, I>(buffs: I) -> String
where
    I: Iterator<Item = &'a Buff>,
{
    buffs
        .map(|item| format!("{:?}({}%)", item, item.percent()))
        .collect::<Vec<String>>()
        .join(" and ")
}

struct SuitePrint {}

impl SuitePrint {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        println!("{}", START_SUITE_MARKER);
        println!("Name: {}", name);
        println!("Description: {}", description);
        Self {}
    }
}

impl Drop for SuitePrint {
    fn drop(&mut self) {
        println!("{}", END_SUITE_MARKER);
    }
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
        let num_buffs = sim
            .buffs()
            .iter()
            .filter_map(|item| match item {
                SlotState::Free(b) | SlotState::Locked(b) => Some(b),
                _ => None,
            })
            .count();
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
fn sim_want_buffs(want: &[Buff]) {
    let want: HashSet<Buff> = HashSet::from_iter(want.iter().copied());
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
        buffs_to_string(want.iter()),
        hits as f64 / attempts as f64 * 100.0
    );
}

// A suite of simluations.
// There are two types of buffs, those that appear 10% of the time and those that appear 12%.
// This suite runs a few simluations.
// Simulation that aims for 10% and 10% buffs.
// Simulation that aims for 10% and 12% buffs.
// Simulation that aims for 12% and 12% buffs.
pub fn suite_desired_buff_percent() {
    let _suite_print = SuitePrint::new(
        "desired buffs",
        "The following tests report how likely (probability) two or more desired buffs appear.",
    );
    // 10%, 10%
    sim_want_buffs(&[Buff::Attack, Buff::Elemental]);
    // 10%, 12%
    sim_want_buffs(&[Buff::Attack, Buff::MaxAmmo]);
    // 12%, 12%
    sim_want_buffs(&[Buff::ChargeDamage, Buff::ChargeSpeed]);

    // 10%, 10%, 10%
    sim_want_buffs(&[Buff::Attack, Buff::Elemental, Buff::CritDamage]);
    // 10%, 10%, 12%
    sim_want_buffs(&[Buff::Attack, Buff::Elemental, Buff::ChargeSpeed]);
    // 10%, 12%, 12%
    sim_want_buffs(&[Buff::Attack, Buff::MaxAmmo, Buff::ChargeSpeed]);
    // 12%, 12%, 12%
    sim_want_buffs(&[Buff::ChargeDamage, Buff::MaxAmmo, Buff::ChargeSpeed]);
}

// Rerolls without locking. Rerolls until all the buffs within |want| is
// rolled.
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

// Rerolls with locking. Rerolls until all the buffs within |want| is
// rolled. If it rolls a wanted buff, it locks immediately.
fn reroll_until_all_found_with_locking(sim: &mut Simulation, want: &HashSet<Buff>) {
    loop {
        sim.reroll();
        let mut found = HashSet::new();

        for b in want.iter() {
            if sim.has_buff(b) {
                let pos = sim.position_of(b).unwrap();
                sim.lock(pos);
                found.insert(*b);
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
fn simulation_num_custom_modules_for_specific_buffs(want: &[Buff]) {
    let want: HashSet<Buff> = HashSet::from_iter(want.iter().copied());
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {} for {attempts} times: \n\
        \t{} custom moduies were used.\n\
        \tThat is on average {} modules.",
        buffs_to_string(want.iter()),
        sum_custom_mods,
        sum_custom_mods as f64 / attempts as f64
    );
}

// See SuitePrint below for description.
pub fn suite_two_desired_buffs_custom_mod_usage() {
    let _suite_print = SuitePrint::new(
        "two desired buffs custom mod usage.",
        "The following tests report how many custom modules \
        were used to get the buffs. None of the buffs are locked during the \
        process.",
    );
    simulation_num_custom_modules_for_specific_buffs(&[Buff::Attack, Buff::Elemental]);
    simulation_num_custom_modules_for_specific_buffs(&[Buff::Attack, Buff::MaxAmmo]);
    simulation_num_custom_modules_for_specific_buffs(&[Buff::ChargeDamage, Buff::ChargeSpeed]);
}

// Simulate to see how many custom modules are required to get a specific set of buffs, with locking.
pub fn simulation_num_cus_mods_with_locking(want: &[Buff]) {
    let want: HashSet<Buff> = HashSet::from_iter(want.iter().copied());
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found_with_locking(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {} for {} times:\n\
        \t{} custom mods were used.\n\
        \tThat is on average {} modules .",
        buffs_to_string(want.iter()),
        attempts,
        sum_custom_mods,
        sum_custom_mods as f64 / attempts as f64
    );
}

pub fn suite_two_desired_buffs_custom_mod_usage_with_locking() {
    let _suite_print = SuitePrint::new(
        "With locking: two desired buffs custom mod usage.",
        "The following tests report how many custom modules \
        were used to get the buffs. When a desired buff appears, they are immediately locked. \
        The cost of locking a module (2+) is accounted.",
    );
    simulation_num_cus_mods_with_locking(&[Buff::Attack, Buff::Elemental]);
    simulation_num_cus_mods_with_locking(&[Buff::Attack, Buff::MaxAmmo]);
    simulation_num_cus_mods_with_locking(&[Buff::ChargeDamage, Buff::ChargeSpeed]);
}

/// Find custom module usage given that a desired buff is locked on the first slot.
///
/// #Arguments
/// * `locked_buff` - Desired buff. The simulation runs given that this buff is locked.
/// * `want_rest` - The list wanted buffs. It is safe to exclude the buff specified in
///                `locked_buff`.
pub fn simulation_first_desired_buff_locked(locked_buff: Buff, want_rest: &[Buff]) {
    simulation_with_locked_buff(locked_buff, 0, want_rest)
}

pub fn suite_simulation_first_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "Simulate first desired buff locked",
        "The following tests report how many custom modules \
        were used to get the desired buffs. \
        This assumes that the desired buff appeard on the FIRST slot on the first roll and \
        locked immediately. \
        When a desired buff appears, they are immediately locked. \
        The cost of locking a module (2+) is accounted.",
    );

    // Two buffs.
    simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental]);
    simulation_first_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo]);
    simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack]);
    simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::ChargeSpeed]);

    // Three buffs.
    // 10%, [10%, 10%].
    simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::CritDamage]);
    // 10%, [10%, 12%].
    simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::MaxAmmo]);
    // 10%, [12%, 12%].
    simulation_first_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo, Buff::CritDamage]);

    // 12%, [10%, 10%].
    simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::Elemental]);
    // 12%, [10%, 12%].
    simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::MaxAmmo]);
    // 12%, [12%, 12%].
    simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::MaxAmmo, Buff::ChargeSpeed]);
}

// Find custom module usage given that a desired buff is locked on the second slot.
pub fn simulation_second_desired_buff_locked(locked_buff: Buff, want_rest: &[Buff]) {
    simulation_with_locked_buff(locked_buff, 1, want_rest)
}

pub fn suite_simulation_second_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "Simulate second desired buff locked",
        "The following tests report how many custom modules \
        were used to get the desired buffs. \
        This assumes that the desired buff appeard on the SECOND slot on the first roll and \
        locked immediately. \
        When a desired buff appears, they are immediately locked. \
        The cost of locking a module (2+) is accounted.",
    );

    // Two buffs.
    simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental]);
    simulation_second_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo]);
    simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack]);
    simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::ChargeSpeed]);

    // Three buffs.
    // 10%, [10%, 10%].
    simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::CritDamage]);
    // 10%, [10%, 12%].
    simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::MaxAmmo]);
    // 10%, [12%, 12%].
    simulation_second_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo, Buff::CritDamage]);

    // 12%, [10%, 10%].
    simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::Elemental]);
    // 12%, [10%, 12%].
    simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::MaxAmmo]);
    // 12%, [12%, 12%].
    simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::MaxAmmo, Buff::ChargeSpeed]);
}

pub fn simulation_with_locked_buff(locked_buff: Buff, position: usize, want_rest: &[Buff]) {
    let want: HashSet<Buff> = HashSet::from_iter(want_rest.iter().copied());
    let attempts = 100000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        // Rolling first so that it uses a module.
        sim.reroll();
        sim.set_buff(position, &locked_buff);
        sim.lock(position);

        reroll_until_all_found_with_locking(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "With {} locked on slot {} \
        plus getting {} for {} times:\n\
        \t{} custom mods were used.\n\
        \tThat is on average {} modules .",
        buffs_to_string([locked_buff].iter()),
        position + 1,
        buffs_to_string(want.iter()),
        attempts,
        sum_custom_mods,
        sum_custom_mods as f64 / attempts as f64
    );
}

pub fn suite_simulation_third_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "Simulate third desired buff locked",
        "The following tests report how many custom modules \
        were used to get the desired buffs. \
        This assumes that the desired buff appeard on the THIRD slot on the first roll and \
        locked immediately. \
        When a desired buff appears, they are immediately locked. \
        The cost of locking a module (2+) is accounted.",
    );

    const SLOT_POSITION: usize = 2;

    // Two buffs.
    simulation_with_locked_buff(Buff::Attack, SLOT_POSITION, &[Buff::Elemental]);
    simulation_with_locked_buff(Buff::Attack, SLOT_POSITION, &[Buff::MaxAmmo]);
    simulation_with_locked_buff(Buff::ChargeDamage, SLOT_POSITION, &[Buff::Attack]);
    simulation_with_locked_buff(Buff::ChargeDamage, SLOT_POSITION, &[Buff::ChargeSpeed]);

    // Three buffs.
    // 10%, [10%, 10%].
    simulation_with_locked_buff(
        Buff::Attack,
        SLOT_POSITION,
        &[Buff::Elemental, Buff::CritDamage],
    );
    // 10%, [10%, 12%].
    simulation_with_locked_buff(
        Buff::Attack,
        SLOT_POSITION,
        &[Buff::Elemental, Buff::MaxAmmo],
    );
    // 10%, [12%, 12%].
    simulation_with_locked_buff(
        Buff::Attack,
        SLOT_POSITION,
        &[Buff::MaxAmmo, Buff::CritDamage],
    );

    // 12%, [10%, 10%].
    simulation_with_locked_buff(
        Buff::ChargeDamage,
        SLOT_POSITION,
        &[Buff::Attack, Buff::Elemental],
    );
    // 12%, [10%, 12%].
    simulation_with_locked_buff(
        Buff::ChargeDamage,
        SLOT_POSITION,
        &[Buff::Attack, Buff::MaxAmmo],
    );
    // 12%, [12%, 12%].
    simulation_with_locked_buff(
        Buff::ChargeDamage,
        SLOT_POSITION,
        &[Buff::MaxAmmo, Buff::ChargeSpeed],
    );
}

#[cfg(test)]
mod test {

    use more_asserts::assert_gt;

    use super::*;

    // Verify that superset of wanted buffs stops the loop.
    #[test]
    fn reroll_until_found_check_is_superset() {
        let mut pass = false;

        // Expect it to occur in 10000 attempts.
        for _ in 0..10000 {
            let mut sim = Simulation::new();
            let want = HashSet::from_iter([Buff::Attack]);
            reroll_until_all_found(&mut sim, &want);

            let buffs: Vec<_> = sim
                .buffs()
                .iter()
                .filter_map(|item| match item {
                    SlotState::Free(buff) | SlotState::Locked(buff) => Some(buff),
                    _ => None,
                })
                .collect();

            // Want a case where the buffs are a strict superset. Retry if the number of buffs
            // is equal to the wanted buffs.
            if buffs.len() <= want.len() {
                continue;
            }

            assert_gt!(buffs.len(), want.len());

            // Make sure that buffs is a superset of want.
            for buff in want.iter() {
                assert!(buffs.contains(&buff));
            }
            pass = true;
            break;
        }

        assert!(pass);
    }
}
