use std::collections::HashSet;

use enum_assoc::Assoc;
use more_asserts::assert_lt;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod simulation;

#[derive(Assoc, EnumIter, Clone, Debug, PartialEq, Eq, Hash)]
#[func(pub fn percent(&self) -> f64)]
enum Buff {
    #[assoc(percent = 10.0)]
    Elemental,
    #[assoc(percent = 12.0)]
    HitRate,
    #[assoc(percent = 12.0)]
    MaxAmmo,
    #[assoc(percent = 10.0)]
    Attack,
    #[assoc(percent = 12.0)]
    ChargeDamage,
    #[assoc(percent = 12.0)]
    ChargeSpeed,
    #[assoc(percent = 12.0)]
    CritRate,
    #[assoc(percent = 10.0)]
    CritDamage,
    #[assoc(percent = 10.0)]
    Defense,
}

#[derive(PartialEq)]
enum AdditionalSlots {
    None,
    SecondOnly,
    ThirdOnly,
    SecondAndThird,
}

enum SlotState {
    // There are no values in the slot.
    None,
    // The buff may change on re-roll.
    Free(Buff),
    // The buff will not change on reroll.
    Locked(Buff),
}

struct Simulation {
    custom_modules: i32,
    attempts: i32,
    buffs: Vec<SlotState>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            custom_modules: 0,
            attempts: 0,
            buffs: vec![SlotState::None, SlotState::None, SlotState::None],
        }
    }

    pub fn buffs(&self) -> &Vec<SlotState> {
        &self.buffs
    }

    pub fn simulate(&mut self) {
        self.custom_modules = 1;
        self.attempts = 1;
        let result = simulate_once();
        let result = result
            .into_iter()
            .map(|item| {
                if let Some(v) = item {
                    return SlotState::Free(v);
                }
                return SlotState::None;
            })
            .collect::<Vec<_>>();
        self.buffs = result;
    }

    /// Reroll the buffs. Locked buffs will not change, and will use more custom modules accordingly.
    pub fn reroll(&mut self) {
        let lock_count = self.buffs.iter().fold(0, |accum, item| {
            if let SlotState::Locked(_) = item {
                return accum + 1;
            }
            accum
        });

        let cus_mod_usage = lock_count + 1;

        self.custom_modules += cus_mod_usage;
        self.attempts += 1;
        self.reroll_buffs();
    }

    // First this collects all locked buffs so that it does not appear in the next draw.
    //
    // The rest is the same as initial roll. Except that the locked ones are skipped.
    fn reroll_buffs(&mut self) {
        let locked_buffs: HashSet<&Buff> = self
            .buffs
            .iter()
            .filter_map(|item| match item {
                SlotState::Locked(buff) => Some(buff),
                _ => None,
            })
            .collect();

        let mut buffs: Vec<Buff> = Buff::iter().collect();
        buffs.retain(|item| !locked_buffs.contains(item));

        match self.buffs[0] {
            SlotState::None | SlotState::Free(_) => {
                let first_buff = choose(&buffs);
                buffs.retain(|b| b != &first_buff);
                self.buffs[0] = SlotState::Free(first_buff);
            }
            SlotState::Locked(_) => {
                // do nothing when locked.
            }
        }
        let extra_slots = additional_slots();
        if extra_slots == AdditionalSlots::None {
            return;
        }

        match self.buffs[1] {
            SlotState::None | SlotState::Free(_) => {
                // Note that the match is for the second buff slot. So this should not affect
                // the third buff slot.
                if extra_slots == AdditionalSlots::SecondOnly
                    || extra_slots == AdditionalSlots::SecondAndThird
                {
                    let another_buff = choose(&buffs);
                    buffs.retain(|b| b != &another_buff);
                    self.buffs[1] = SlotState::Free(another_buff);
                }
            }
            SlotState::Locked(_) => {}
        }

        if extra_slots == AdditionalSlots::SecondOnly {
            return;
        }

        match self.buffs[2] {
            SlotState::None | SlotState::Free(_) => {
                if extra_slots == AdditionalSlots::ThirdOnly
                    || extra_slots == AdditionalSlots::SecondAndThird
                {
                    let another_buff = choose(&buffs);
                    buffs.retain(|b| b != &another_buff);
                    self.buffs[2] = SlotState::Free(another_buff);
                }
            }
            SlotState::Locked(_) => {}
        }
    }

    pub fn has_buff(&self, buff: &Buff) -> bool {
        self.buffs.iter().any(|state| match state {
            SlotState::Free(b) | SlotState::Locked(b) => buff == b,
            _ => false,
        })
    }

    fn lock(&mut self, pos: usize) {
        assert_lt!(pos, self.buffs.len());
        if let SlotState::Free(buff) = &self.buffs[pos] {
            self.buffs[pos] = SlotState::Locked(buff.clone());
        }
    }

    pub fn lock_first(&mut self) {
        self.lock(0);
    }

    pub fn lock_second(&mut self) {
        self.lock(1);
    }

    pub fn lock_third(&mut self) {
        self.lock(2);
    }
}

fn choose(buffs: &[Buff]) -> Buff {
    let sum: f64 = buffs.iter().map(|b| b.percent()).sum();

    let mut rng = rand::thread_rng();
    let value = rng.gen_range(0.0..sum);

    let mut accum = 0.0;
    for b in buffs.iter() {
        let next_threshold = accum + b.percent();
        if value < next_threshold {
            return b.clone();
        }
        accum = next_threshold;
    }

    panic!("With correct percentage calculation, it should not reach here!");
}

fn additional_slots() -> AdditionalSlots {
    let mut rng = rand::thread_rng();

    // Draw for second slot.
    let value = rng.gen_range(0.0..100.0);
    let mut gets_second_slot = false;
    if value < 50.0 {
        gets_second_slot = true;
    }

    // Draw for third slot.
    let value = rng.gen_range(0.0..100.0);
    let mut gets_third_slot = false;
    if value < 30.0 {
        gets_third_slot = true;
    }

    if gets_second_slot && gets_third_slot {
        return AdditionalSlots::SecondAndThird;
    } else if gets_second_slot {
        return AdditionalSlots::SecondOnly;
    } else if gets_third_slot {
        return AdditionalSlots::ThirdOnly;
    }
    AdditionalSlots::None
}

/// Returns a list of buffs for one off simulation.
///
/// The return value will always be 3 elements.
/// The first element always contains a value.
/// The second or third element may contain values. Also third element may be persent by not
/// the second. E.g. (Buff1, None, Buff2).
fn simulate_once() -> Vec<Option<Buff>> {
    let mut drawn_buffs = vec![];
    let mut buffs: Vec<Buff> = Buff::iter().collect();

    let extra_slots = additional_slots();
    let first_buff = choose(&buffs);
    buffs.retain(|b| b != &first_buff);
    drawn_buffs.push(Some(first_buff));

    if extra_slots == AdditionalSlots::None {
        drawn_buffs.push(None);
        drawn_buffs.push(None);
        return drawn_buffs;
    }

    let another_buff = choose(&buffs);
    buffs.retain(|b| b != &another_buff);
    if extra_slots == AdditionalSlots::SecondOnly {
        drawn_buffs.push(Some(another_buff));
        drawn_buffs.push(None);
        return drawn_buffs;
    }

    if extra_slots == AdditionalSlots::ThirdOnly {
        drawn_buffs.push(None);
        drawn_buffs.push(Some(another_buff));
        return drawn_buffs;
    }

    // Case where all slots filled.
    drawn_buffs.push(Some(another_buff));

    let another_buff = choose(&buffs);
    buffs.retain(|b| b != &another_buff);
    drawn_buffs.push(Some(another_buff));

    drawn_buffs
}

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
fn simulation_want_specific() {
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

// Simulation to see how many custom modules are required to get all buffs without locking.
// For example if you want Attack and ChargeSpeed. This simulation checks how many custom modules
// are used to got you all the buffs that you want.
// This runs the process multiple times and gets the average number of modules required, without
// locking.
// Oddly this does not match with the simulation that gets the probability of getting all buffs.
fn simulation_num_cus_mod_for_specific() {
    let want: HashSet<Buff> = HashSet::from_iter([Buff::Attack, Buff::ChargeSpeed]);
    let attempts = 10000;

    let mut sum_custom_mods = 0;

    for _ in 0..attempts {
        let mut sim = Simulation::new();
        reroll_until_all_found(&mut sim, &want);
        sum_custom_mods += sim.custom_modules;
    }

    println!(
        "To get all {:?} for {attempts} times, it required {} custom mods. That is on average {} rolls.",
        want, sum_custom_mods, 
        sum_custom_mods as f64 / attempts as f64
    );
}


fn main() {
    simulation_slots_shown_distribution();
    println!("sim want specific");
    simulation_want_specific();
    println!("sim num cusmods");
    simulation_num_cus_mod_for_specific();
}

#[cfg(test)]
mod test {

    use itertools::{all, Itertools};
    use more_asserts::{assert_ge, assert_le};
    use std::{collections::HashMap, vec};

    use super::*;

    #[test]
    fn check_distribution_10000() {
        let mut samples = vec![];
        let buffs: Vec<Buff> = Buff::iter().collect();
        for _ in (0..10000) {
            let buff = choose(&buffs);
            samples.push(buff);
        }

        let counts: HashMap<Buff, usize> = samples.into_iter().counts();
        let range_for_10_percent = 500..1500;

        assert!(range_for_10_percent.contains(&counts[&Buff::Elemental]));
        assert!(range_for_10_percent.contains(&counts[&Buff::Attack]));
        assert!(range_for_10_percent.contains(&counts[&Buff::CritDamage]));
        assert!(range_for_10_percent.contains(&counts[&Buff::Defense]));

        let range_for_12_percent = 600..2000;
        assert!(range_for_12_percent.contains(&counts[&Buff::HitRate]));
        assert!(range_for_12_percent.contains(&counts[&Buff::MaxAmmo]));
        assert!(range_for_12_percent.contains(&counts[&Buff::ChargeDamage]));
        assert!(range_for_12_percent.contains(&counts[&Buff::ChargeSpeed]));
        assert!(range_for_12_percent.contains(&counts[&Buff::CritRate]));
    }

    #[test]
    fn check_partial_choices() {
        let choices = [Buff::Attack, Buff::Elemental];
        let samples: Vec<Buff> = (0..10000).map(|_| choose(&choices)).collect();

        let counts: HashMap<Buff, usize> = samples.into_iter().counts();
        assert_eq!(counts.len(), 2);
        assert!(counts.contains_key(&Buff::Attack));
        assert!(counts.contains_key(&Buff::Elemental));
    }

    #[test]
    fn sim_once() {
        for _ in 0..10000 {
            let r = simulate_once();
            assert_eq!(r.len(), 3);
        }
    }

    #[test]
    fn can_happen_three_slots() {
        for _ in 0..10000 {
            let r = simulate_once();
            assert_eq!(r.len(), 3);

            if r[0].is_some() && r[1].is_some() && r[2].is_some() {
                return;
            }
        }
        panic!("Should have seen three slots.");
    }

    #[test]
    fn can_happen_second_slot_missing() {
        for _ in 0..10000 {
            let r = simulate_once();
            assert_eq!(r.len(), 3);

            if r[0].is_some() && r[1].is_none() && r[2].is_some() {
                return;
            }
        }
        panic!("Should have seen second slot missing.");
    }

    #[test]
    fn can_happen_third_slot_missing() {
        for _ in 0..10000 {
            let r = simulate_once();
            assert_eq!(r.len(), 3);

            if r[0].is_some() && r[1].is_some() && r[2].is_none() {
                return;
            }
        }
        panic!("Should have seen third slot missing.");
    }

    #[test]
    fn first_slot_always_filled() {
        for _ in 0..10000 {
            let r = simulate_once();
            assert_eq!(r.len(), 3);
            assert!(r[0].is_some());
        }
    }

    #[test]
    fn simulation_reroll_init() {
        let mut sim = Simulation::new();
        sim.reroll();

        assert_eq!(sim.attempts, 1);
        assert_eq!(sim.custom_modules, 1);
        assert_eq!(sim.buffs().len(), 3);

        assert!(matches!(
            sim.buffs[0],
            SlotState::Locked(_) | SlotState::Free(_)
        ));
    }

    // Verify that the distribution of the buffs are OK.
    #[test]
    fn simulation_reroll_multiiple() {
        let mut sim = Simulation::new();
        let mut all_buffs = vec![];
        for _ in 0..10000 {
            sim.reroll();
            let buffs: Vec<_> = sim
                .buffs()
                .iter()
                .filter_map(|item| match item {
                    SlotState::Free(b) => Some(b.clone()),
                    SlotState::Locked(b) => Some(b.clone()),
                    SlotState::None => None,
                })
                .collect();

            all_buffs.extend(buffs);
        }

        let all_buffs_len = all_buffs.len();

        let counts: HashMap<Buff, usize> = all_buffs.into_iter().counts();
        println!("counts: {:?}", counts);

        // For 10% buffs, assume they occur about 7%-13% of all buffs.
        let seven_percent = (0.07 * all_buffs_len as f64) as usize;
        let thirteen_percent = (0.13 * all_buffs_len as f64) as usize;

        assert_ge!(counts[&Buff::Elemental], seven_percent);
        assert_le!(counts[&Buff::Elemental], thirteen_percent);
        assert_ge!(counts[&Buff::Attack], seven_percent);
        assert_le!(counts[&Buff::Attack], thirteen_percent);
        assert_ge!(counts[&Buff::CritDamage], seven_percent);
        assert_le!(counts[&Buff::CritDamage], thirteen_percent);
        assert_ge!(counts[&Buff::Defense], seven_percent);
        assert_le!(counts[&Buff::Defense], thirteen_percent);

        // For 12% buffs, assume they occur about 9%-15% of all buffs.
        let nine_percent = (0.09 * all_buffs_len as f64) as usize;
        let fifteen_percent = (0.15 * all_buffs_len as f64) as usize;

        assert_ge!(counts[&Buff::HitRate], nine_percent);
        assert_le!(counts[&Buff::HitRate], fifteen_percent);
        assert_ge!(counts[&Buff::MaxAmmo], nine_percent);
        assert_le!(counts[&Buff::MaxAmmo], fifteen_percent);
        assert_ge!(counts[&Buff::ChargeDamage], nine_percent);
        assert_le!(counts[&Buff::ChargeDamage], fifteen_percent);
        assert_ge!(counts[&Buff::ChargeSpeed], nine_percent);
        assert_le!(counts[&Buff::ChargeSpeed], fifteen_percent);
        assert_ge!(counts[&Buff::CritRate], nine_percent);
        assert_le!(counts[&Buff::CritRate], fifteen_percent);
    }

    #[test]
    fn simulation_reroll_custom_module_count() {
        let mut sim = Simulation::new();
        for _ in 0..1000 {
            sim.reroll();
        }

        assert_eq!(sim.attempts, 1000);
        assert_eq!(sim.custom_modules, 1000);
    }

    #[test]
    fn has_buff() {
        let mut sim = Simulation::new();
        sim.reroll();

        let SlotState::Free(buff) = &sim.buffs()[0] else {
            panic!("First slot on first roll must be free state.");
        };

        assert!(sim.has_buff(buff));
    }
}
