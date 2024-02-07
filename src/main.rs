use std::collections::HashSet;

use enum_assoc::Assoc;
use rand::Rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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

struct Simulation {
    pub custom_modules: i32,
    pub attempts: i32,
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

pub fn simulation_slots_shown_distribution() {
    let attempts = 10000000;

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

fn simulation_want_specific() {
    let want: HashSet<Buff> = [Buff::Attack, Buff::ChargeSpeed].into_iter().collect();
    let attempts = 10000000;

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

fn simulation_num_cus_mod_for_specific() {
    let want: HashSet<Buff> = [Buff::Attack, Buff::ChargeSpeed].into_iter().collect();
    let attempts = 10000000;

    let mut hits = 0;
    for _ in 0..attempts {
        let sim_result = simulate_once();
    }

    println!(
        "Out of {attempts}, {hits} containted the wanted buffs. That is {}%",
        hits as f64 / attempts as f64 * 100.0
    );
}

fn main() {
    simulation_want_specific();
}

#[cfg(test)]
mod test {

    use itertools::Itertools;
    use std::collections::HashMap;

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
        assert!(range_for_12_percent.contains(&counts[&Buff::ChargeDamage]));
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
}
