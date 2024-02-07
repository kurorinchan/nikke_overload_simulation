use enum_assoc::Assoc;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

#[derive(Assoc, EnumIter, Clone, Debug)]
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

fn main() {
    let want = [Buff::Attack, Buff::ChargeSpeed];

    let mut buffs: Vec<Buff> = Buff::iter().collect();
    for _ in (0..10) {
        let buff = choose(&buffs);
        println!("{:?}", buff);
    }
}
