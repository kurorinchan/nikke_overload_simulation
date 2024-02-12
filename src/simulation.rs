use tabled::{
    builder::Builder,
    settings::{style::Style, themes::ColumnNames},
};

use crate::*;

use hdrhistogram::Histogram;

const DEFAULT_ATTEMPTS: u32 = 100000;

const START_SUITE_MARKER: &str = "===== SUITE START =====";
const END_SUITE_MARKER: &str = "=====  SUITE END  =====";

fn buff_to_string(buff: Buff) -> String {
    buffs_to_string([buff].iter())
}

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
    pub fn new(name: &str, description: &str) -> Self {
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

pub struct SimulationResult {
    locked_buff: Option<Buff>,
    buffs: Vec<Buff>,
    modules: Vec<u64>,
    histogram: Histogram<u32>,
}

impl SimulationResult {
    pub fn new() -> Self {
        const SIGNIFICANT_FIGURES: u8 = 3;
        SimulationResult {
            locked_buff: None,
            buffs: vec![],
            modules: vec![],
            histogram: Histogram::<u32>::new(SIGNIFICANT_FIGURES).unwrap(),
        }
    }

    pub fn record(&mut self, data: u32) {
        self.histogram += data as u64;
        self.modules.push(data as u64);
    }

    pub fn mean(&self) -> f64 {
        self.histogram.mean()
    }

    pub fn stddev(&self) -> f64 {
        self.histogram.stdev()
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
fn simulation_num_custom_modules_for_specific_buffs(want: &[Buff]) -> SimulationResult {
    let mut result = SimulationResult::new();
    result.buffs.extend(want.iter());

    let want: HashSet<Buff> = HashSet::from_iter(want.iter().copied());

    for _ in 0..DEFAULT_ATTEMPTS {
        let mut sim = Simulation::new();
        reroll_until_all_found(&mut sim, &want);
        result.record(sim.custom_modules);
    }

    result
}

// See SuitePrint below for description.
pub fn suite_two_desired_buffs_custom_mod_usage() {
    let _suite_print = SuitePrint::new(
        "two desired buffs custom mod usage.",
        format!(
            "The following table shows how many custom modules \
        were used to get the listed buffs. The simulations ran until there were rolls that got all
        the listed buffs for {} times. \
        None of the buffs are locked during the process.",
            DEFAULT_ATTEMPTS
        )
        .as_str(),
    );

    let mut data = vec![vec![
        "buff1".to_string(),
        "buff2".to_string(),
        "mean (modules)".to_string(),
        "std dev (modules)".to_string(),
    ]];

    let results = [
        simulation_num_custom_modules_for_specific_buffs(&[Buff::Attack, Buff::Elemental]),
        simulation_num_custom_modules_for_specific_buffs(&[Buff::Attack, Buff::MaxAmmo]),
        simulation_num_custom_modules_for_specific_buffs(&[Buff::ChargeDamage, Buff::ChargeSpeed]),
    ];

    for result in results {
        data.push(vec![
            buff_to_string(result.buffs[0]),
            buff_to_string(result.buffs[1]),
            format!("{:.3}", result.mean()),
            format!("{:.3}", result.stddev()),
        ]);
    }

    let mut table = Builder::from(data).build();
    table.with(Style::modern());
    println!("{table}");
}

// Simulate to see how many custom modules are required to get a specific set of buffs, with locking.
pub fn simulation_num_cus_mods_with_locking(want: &[Buff]) -> SimulationResult {
    let mut result = SimulationResult::new();
    result.buffs.extend(want.iter());

    let want: HashSet<Buff> = HashSet::from_iter(want.iter().copied());
    let attempts = 100000;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found_with_locking(&mut sim, &want);
        result.record(sim.custom_modules);
    }
    result
}

pub fn suite_two_desired_buffs_custom_mod_usage_with_locking() {
    let _suite_print = SuitePrint::new(
        "With locking: two desired buffs custom mod usage.",
        "The following tests report how many custom modules \
        were used to get the buffs. When a desired buff appears, they are immediately locked. \
        The cost of locking a module (2+) is accounted.",
    );

    let mut data = vec![vec![
        "buff1".to_string(),
        "buff2".to_string(),
        "mean (modules)".to_string(),
        "std dev (modules)".to_string(),
    ]];

    let results = [
        simulation_num_cus_mods_with_locking(&[Buff::Attack, Buff::Elemental]),
        simulation_num_cus_mods_with_locking(&[Buff::Attack, Buff::MaxAmmo]),
        simulation_num_cus_mods_with_locking(&[Buff::ChargeDamage, Buff::ChargeSpeed]),
    ];

    for result in results {
        data.push(vec![
            buffs_to_string([result.buffs[0]].iter()),
            buffs_to_string([result.buffs[1]].iter()),
            format!("{:.3}", result.mean()),
            format!("{:.3}", result.stddev()),
        ]);
    }

    let mut table = Builder::from(data).build();
    table.with(Style::modern());
    println!("{table}");
}

/// Find custom module usage given that a desired buff is locked on the first slot.
///
/// #Arguments
/// * `locked_buff` - Desired buff. The simulation runs given that this buff is locked.
/// * `want_rest` - The list wanted buffs. It is safe to exclude the buff specified in
///                `locked_buff`.
pub fn simulation_first_desired_buff_locked(
    locked_buff: Buff,
    want_rest: &[Buff],
) -> SimulationResult {
    simulation_with_locked_buff(locked_buff, 0, want_rest)
}

pub fn suite_simulation_first_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "First slot buff locked",
        format!(
            "The following table shows how many custom modules \
        were used to get the desired buffs. \
        Given that the FIRST slot has been locked after the first roll \
        (1 for roll + 2 for locking), \
        whenever a desired buff appears on a reroll, it is immediately locked. \
        The cost of locking modules (2+) is accounted. \
        The simluation rerolls until all the preferred buffs are drawn. And each row shows the \
        statistics on the number of custom modules used until the preferred buffs are drawn \
        {} times.",
            DEFAULT_ATTEMPTS
        )
        .as_str(),
    );

    let mut data = vec![vec![
        "locked buff (slot 1)".to_string(),
        "buff1".to_string(),
        "buff2".to_string(),
        "mean (modules)".to_string(),
        "std dev (modules)".to_string(),
    ]];

    // Two buffs.
    let results = [
        simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental]),
        simulation_first_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo]),
        simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack]),
        simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::ChargeSpeed]),
        // Three buffs.
        // 10%, [10%, 10%].
        simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::CritDamage]),
        // 10%, [10%, 12%].
        simulation_first_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::MaxAmmo]),
        // 10%, [12%, 12%].
        simulation_first_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo, Buff::CritDamage]),
        // 12%, [10%, 10%].
        simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::Elemental]),
        // 12%, [10%, 12%].
        simulation_first_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::MaxAmmo]),
        // 12%, [12%, 12%].
        simulation_first_desired_buff_locked(
            Buff::ChargeDamage,
            &[Buff::MaxAmmo, Buff::ChargeSpeed],
        ),
    ];

    for result in results {
        let second_buff = if result.buffs.len() < 2 {
            "N/A".to_string()
        } else {
            buff_to_string(result.buffs[1])
        };

        data.push(vec![
            buffs_to_string([result.locked_buff.unwrap()].iter()),
            buffs_to_string([result.buffs[0]].iter()),
            second_buff,
            format!("{:.3}", result.mean()),
            format!("{:.3}", result.stddev()),
        ]);
    }

    let mut table = Builder::from(data).build();
    table.with(Style::modern());
    println!("{table}");
}

// Find custom module usage given that a desired buff is locked on the second slot.
pub fn simulation_second_desired_buff_locked(
    locked_buff: Buff,
    want_rest: &[Buff],
) -> SimulationResult {
    simulation_with_locked_buff(locked_buff, 1, want_rest)
}

pub fn suite_simulation_second_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "Second slot buff locked",
        format!(
            "The following table shows how many custom modules \
        were used to get the desired buffs. \
        Given that the SECOND slot has been locked after the first roll \
        (1 for roll + 2 for locking), \
        whenever a desired buff appears on a reroll, it is immediately locked. \
        The cost of locking modules (2+) is accounted. \
        The simluation rerolls until all the preferred buffs are drawn. And each row shows the \
        statistics on the number of custom modules used until the preferred buffs are drawn \
        {} times.",
            DEFAULT_ATTEMPTS
        )
        .as_str(),
    );

    let mut data = vec![vec![
        "locked buff (slot 2)".to_string(),
        "buff1".to_string(),
        "buff2".to_string(),
        "mean (modules)".to_string(),
        "std dev (modules)".to_string(),
    ]];

    let results = [
        // Two buffs.
        simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental]),
        simulation_second_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo]),
        simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack]),
        simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::ChargeSpeed]),
        // Three buffs.
        // 10%, [10%, 10%].
        simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::CritDamage]),
        // 10%, [10%, 12%].
        simulation_second_desired_buff_locked(Buff::Attack, &[Buff::Elemental, Buff::MaxAmmo]),
        // 10%, [12%, 12%].
        simulation_second_desired_buff_locked(Buff::Attack, &[Buff::MaxAmmo, Buff::CritDamage]),
        // 12%, [10%, 10%].
        simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::Elemental]),
        // 12%, [10%, 12%].
        simulation_second_desired_buff_locked(Buff::ChargeDamage, &[Buff::Attack, Buff::MaxAmmo]),
        // 12%, [12%, 12%].
        simulation_second_desired_buff_locked(
            Buff::ChargeDamage,
            &[Buff::MaxAmmo, Buff::ChargeSpeed],
        ),
    ];

    for result in results {
        let second_buff = if result.buffs.len() < 2 {
            "N/A".to_string()
        } else {
            buff_to_string(result.buffs[1])
        };

        data.push(vec![
            buffs_to_string([result.locked_buff.unwrap()].iter()),
            buffs_to_string([result.buffs[0]].iter()),
            second_buff,
            format!("{:.3}", result.mean()),
            format!("{:.3}", result.stddev()),
        ]);
    }

    let mut table = Builder::from(data).build();
    table.with(Style::modern());
    println!("{table}");
}

pub fn simulation_with_locked_buff(
    locked_buff: Buff,
    position: usize,
    want_rest: &[Buff],
) -> SimulationResult {
    let mut result = SimulationResult::new();
    result.buffs.extend(want_rest.iter());
    result.locked_buff = Some(locked_buff);

    let want: HashSet<Buff> = HashSet::from_iter(want_rest.iter().copied());
    let attempts = DEFAULT_ATTEMPTS;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        // Rolling first so that it uses a module.
        sim.reroll();
        sim.set_buff(position, &locked_buff);
        sim.lock(position);

        reroll_until_all_found_with_locking(&mut sim, &want);
        result.record(sim.custom_modules)
    }

    result
}

pub fn suite_simulation_third_slot_buff_locked() {
    let _suite_print = SuitePrint::new(
        "Third slot buff locked",
        format!(
            "The following table shows how many custom modules \
        were used to get the desired buffs. \
        Given that the THIRD slot has been locked after the first roll \
        (1 for roll + 2 for locking), \
        whenever a desired buff appears on a reroll, it is immediately locked. \
        The cost of locking modules (2+) is accounted. \
        The simluation rerolls until all the preferred buffs are drawn. And each row shows the \
        statistics on the number of custom modules used until the preferred buffs are drawn \
        {} times.",
            DEFAULT_ATTEMPTS
        )
        .as_str(),
    );

    const SLOT_POSITION: usize = 2;

    let mut data = vec![vec![
        "locked buff (slot 3)".to_string(),
        "buff1".to_string(),
        "buff2".to_string(),
        "mean (modules)".to_string(),
        "std dev (modules)".to_string(),
    ]];

    let results = [
        // Two buffs.
        simulation_with_locked_buff(Buff::Attack, SLOT_POSITION, &[Buff::Elemental]),
        simulation_with_locked_buff(Buff::Attack, SLOT_POSITION, &[Buff::MaxAmmo]),
        simulation_with_locked_buff(Buff::ChargeDamage, SLOT_POSITION, &[Buff::Attack]),
        simulation_with_locked_buff(Buff::ChargeDamage, SLOT_POSITION, &[Buff::ChargeSpeed]),
        // Three buffs.
        // 10%, [10%, 10%].
        simulation_with_locked_buff(
            Buff::Attack,
            SLOT_POSITION,
            &[Buff::Elemental, Buff::CritDamage],
        ),
        // 10%, [10%, 12%].
        simulation_with_locked_buff(
            Buff::Attack,
            SLOT_POSITION,
            &[Buff::Elemental, Buff::MaxAmmo],
        ),
        // 10%, [12%, 12%].
        simulation_with_locked_buff(
            Buff::Attack,
            SLOT_POSITION,
            &[Buff::MaxAmmo, Buff::CritDamage],
        ),
        // 12%, [10%, 10%].
        simulation_with_locked_buff(
            Buff::ChargeDamage,
            SLOT_POSITION,
            &[Buff::Attack, Buff::Elemental],
        ),
        // 12%, [10%, 12%].
        simulation_with_locked_buff(
            Buff::ChargeDamage,
            SLOT_POSITION,
            &[Buff::Attack, Buff::MaxAmmo],
        ),
        // 12%, [12%, 12%].
        simulation_with_locked_buff(
            Buff::ChargeDamage,
            SLOT_POSITION,
            &[Buff::MaxAmmo, Buff::ChargeSpeed],
        ),
    ];

    for result in results {
        let second_buff = if result.buffs.len() < 2 {
            "N/A".to_string()
        } else {
            buff_to_string(result.buffs[1])
        };

        data.push(vec![
            buffs_to_string([result.locked_buff.unwrap()].iter()),
            buffs_to_string([result.buffs[0]].iter()),
            second_buff,
            format!("{:.3}", result.mean()),
            format!("{:.3}", result.stddev()),
        ]);
    }

    let mut table = Builder::from(data).build();
    table.with(Style::modern());
    println!("{table}");
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
