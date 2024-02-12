# What
This is a simulation for NIKKE's overload gears.

Specifically, this is the source code for running overload gear reroll
simluation. The description in the simluation should be self explanatory, but
the basic idea of the simluatons is to calculate how many custom modules are
required to get desired buffs.

The simluation cases should cover all combinations of buffs that appear 10% and 12%.

# Assumption

Suppose buff A has been chosen for the first slot. When drawing a buff for the second slot,
buff A is removed from the lottery. 

For example suppose the first slot gets the ATTACK buff that
appears 10% of the time. Then the lottery for the second slot will not include the ATTACK buff.
This implies that the "10%" chance for getting the ATTACK buff has been "removed", and therefore
the "10%" chance is distributed evenly to the lottery of other buffs. In other words, suppose
I have a 100-face-die. Faces 1 to 10 is for ATTACK. First roll gave me 5 so ATTACK was drawn. For
the second slot buff, instead of using a 100-face-die, I use 90-face-die 
to draw a buff.

Same for other situations, e.g. locked buffs, third slot, etc.

# Thoughts

Draw your own conclusion by looking at the numbers yourself in the
[result](#result) section. My thoughts are below.

## Casual gaming strategy
* Aim for 2 buffs. It should get you a decent build.
* If you have 20+ modules (maybe 30) in stock, keep rerolling until you get the 2 buffs.
* If you are short on modules but rolled a desireable buff on the first slot, it's OK to lock it.
  * The difference between locking the first slot and not locking is quite small,
  around 2 modules more (in fact the 2 modules is the cost of locking). So if
  you really need the buff now and are short on modules (have no modules to
  reroll) locking it is not a terrible strategy.

## Min-maxing

* Getting 2 buffs (on average) requires about **20** mods. However locking the
  first slot buff may not be very wise especially if you want the second buff to
  be a 10% chance buff (e.g. Attack), just reroll the whole thing instead (27 vs 25).
* Locking the third slot seems wise. Especially if you are aiming for 3 buffs.
  However, getting a desired buff on the third slot may require
  some luck, and rerolling until so may end up costing more.
    * A good rule of thumb is, if you happen to get a desired buff on the third buff within 25
      rolls then it *was* worth it.
    * Keep in mind that the chance of getting any buff on third slot is 30%. And
      if you want ATTACK (10%) buff in that slot, that is about 3% chance. With naive
      math, 33+ rolls are expected to get there.
* Locking the first slot (on average) requires more mods than locking other slots. However, getting
  a desired buff on the second or third slot may require more mods.

# Result
The raw output is below. Running on anyone else's machine should not change the
results significantly.


```
Tally of buff nums! [34939, 50151, 14910]
Percentage of buffs 34.94% 50.15% 14.91%
===== SUITE START =====
Name: desired buffs
Description: The following tests report how likely (probability) two or more desired buffs appear.
To get buffs Attack(10%) and Elemental(10%). The simulation ran 100000 rerolls, which 2142 had all the buffs. This is about 2.14%.
To get buffs MaxAmmo(12%) and Attack(10%). The simulation ran 100000 rerolls, which 2512 had all the buffs. This is about 2.51%.
To get buffs ChargeSpeed(12%) and ChargeDamage(12%). The simulation ran 100000 rerolls, which 3061 had all the buffs. This is about 3.06%.
To get buffs Attack(10%) and Elemental(10%) and CritDamage(10%). The simulation ran 100000 rerolls, which 138 had all the buffs. This is about 0.14%.
To get buffs ChargeSpeed(12%) and Attack(10%) and Elemental(10%). The simulation ran 100000 rerolls, which 174 had all the buffs. This is about 0.17%.
To get buffs Attack(10%) and ChargeSpeed(12%) and MaxAmmo(12%). The simulation ran 100000 rerolls, which 196 had all the buffs. This is about 0.20%.
To get buffs ChargeDamage(12%) and MaxAmmo(12%) and ChargeSpeed(12%). The simulation ran 100000 rerolls, which 212 had all the buffs. This is about 0.21%.
=====  SUITE END  =====
===== SUITE START =====
Name: two desired buffs custom mod usage.
Description: The following table shows how many custom modules were used to get the listed buffs. The simulations ran until there were rolls that got all
        the listed buffs for 100000 times. None of the buffs are locked during the process.
┌───────────────────┬──────────────────┬────────────────┬───────────────────┐
│ buff1             │ buff2            │ mean (modules) │ std dev (modules) │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)       │ Elemental(10%)   │ 23.871         │ 22.261            │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)       │ MaxAmmo(12%)     │ 20.435         │ 18.909            │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%) │ ChargeSpeed(12%) │ 17.592         │ 16.182            │
└───────────────────┴──────────────────┴────────────────┴───────────────────┘
=====  SUITE END  =====
===== SUITE START =====
Name: With locking: two desired buffs custom mod usage.
Description: The following tests report how many custom modules were used to get the buffs. When a desired buff appears, they are immediately locked. The cost of locking a module (2+) is accounted.
┌───────────────────┬──────────────────┬────────────────┬───────────────────┐
│ buff1             │ buff2            │ mean (modules) │ std dev (modules) │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)       │ Elemental(10%)   │ 25.195         │ 18.596            │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)       │ MaxAmmo(12%)     │ 23.440         │ 17.202            │
├───────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%) │ ChargeSpeed(12%) │ 21.488         │ 15.102            │
└───────────────────┴──────────────────┴────────────────┴───────────────────┘
=====  SUITE END  =====
===== SUITE START =====
Name: First slot buff locked
Description: The following table shows how many custom modules were used to get the desired buffs. Given that the FIRST slot has been locked after the first roll (1 for roll + 2 for locking), whenever a desired buff appears on a reroll, it is immediately locked. The cost of locking modules (2+) is accounted. The simluation rerolls until all the preferred buffs are drawn. And each row shows the statistics on the number of custom modules used until the preferred buffs are drawn 100000 times.
┌──────────────────────┬──────────────────┬──────────────────┬────────────────┬───────────────────┐
│ locked buff (slot 1) │ buff1            │ buff2            │ mean (modules) │ std dev (modules) │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ N/A              │ 27.426         │ 21.368            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ N/A              │ 23.833         │ 17.729            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ N/A              │ 26.878         │ 20.781            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ ChargeSpeed(12%) │ N/A              │ 23.439         │ 17.201            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ CritDamage(10%)  │ 82.800         │ 70.894            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ MaxAmmo(12%)     │ 75.468         │ 64.915            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ CritDamage(10%)  │ 75.635         │ 64.273            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ Elemental(10%)   │ 80.680         │ 69.241            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ MaxAmmo(12%)     │ 73.655         │ 63.036            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ MaxAmmo(12%)     │ ChargeSpeed(12%) │ 66.832         │ 55.950            │
└──────────────────────┴──────────────────┴──────────────────┴────────────────┴───────────────────┘
=====  SUITE END  =====
===== SUITE START =====
Name: Second slot buff locked
Description: The following table shows how many custom modules were used to get the desired buffs. Given that the SECOND slot has been locked after the first roll (1 for roll + 2 for locking), whenever a desired buff appears on a reroll, it is immediately locked. The cost of locking modules (2+) is accounted. The simluation rerolls until all the preferred buffs are drawn. And each row shows the statistics on the number of custom modules used until the preferred buffs are drawn 100000 times.
┌──────────────────────┬──────────────────┬──────────────────┬────────────────┬───────────────────┐
│ locked buff (slot 2) │ buff1            │ buff2            │ mean (modules) │ std dev (modules) │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ N/A              │ 19.447         │ 12.608            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ N/A              │ 17.308         │ 10.411            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ N/A              │ 19.183         │ 12.359            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ ChargeSpeed(12%) │ N/A              │ 17.030         │ 10.167            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ CritDamage(10%)  │ 76.793         │ 72.727            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ MaxAmmo(12%)     │ 70.410         │ 66.863            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ CritDamage(10%)  │ 70.473         │ 67.114            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ Elemental(10%)   │ 74.958         │ 71.280            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ MaxAmmo(12%)     │ 69.130         │ 65.830            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ MaxAmmo(12%)     │ ChargeSpeed(12%) │ 61.905         │ 57.536            │
└──────────────────────┴──────────────────┴──────────────────┴────────────────┴───────────────────┘
=====  SUITE END  =====
===== SUITE START =====
Name: Third slot buff locked
Description: The following table shows how many custom modules were used to get the desired buffs. Given that the THIRD slot has been locked after the first roll (1 for roll + 2 for locking), whenever a desired buff appears on a reroll, it is immediately locked. The cost of locking modules (2+) is accounted. The simluation rerolls until all the preferred buffs are drawn. And each row shows the statistics on the number of custom modules used until the preferred buffs are drawn 100000 times.
┌──────────────────────┬──────────────────┬──────────────────┬────────────────┬───────────────────┐
│ locked buff (slot 3) │ buff1            │ buff2            │ mean (modules) │ std dev (modules) │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ N/A              │ 17.676         │ 10.880            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ N/A              │ 15.749         │ 8.949             │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ N/A              │ 17.427         │ 10.632            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ ChargeSpeed(12%) │ N/A              │ 15.562         │ 8.745             │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ CritDamage(10%)  │ 49.706         │ 41.631            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ Elemental(10%)   │ MaxAmmo(12%)     │ 45.510         │ 38.016            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ Attack(10%)          │ MaxAmmo(12%)     │ CritDamage(10%)  │ 45.649         │ 38.064            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ Elemental(10%)   │ 48.543         │ 40.495            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ Attack(10%)      │ MaxAmmo(12%)     │ 44.723         │ 37.293            │
├──────────────────────┼──────────────────┼──────────────────┼────────────────┼───────────────────┤
│ ChargeDamage(12%)    │ MaxAmmo(12%)     │ ChargeSpeed(12%) │ 40.545         │ 32.799            │
└──────────────────────┴──────────────────┴──────────────────┴────────────────┴───────────────────┘
=====  SUITE END  =====
```
