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
Tally of buff nums! [34977, 49976, 15047]
Percentage of buffs 34.98% 49.98% 15.05%
===== SUITE START =====
Name: two desired buffs
Description: The following tests report how likely (probability) two desired buffs appear.
To get buffs Attack(10%) and Elemental(10%). The simulation ran 100000 rerolls, which 2173 had all the buffs. This is about 2.17%.
To get buffs MaxAmmo(12%) and Attack(10%). The simulation ran 100000 rerolls, which 2513 had all the buffs. This is about 2.51%.
To get buffs ChargeDamage(12%) and ChargeSpeed(12%). The simulation ran 100000 rerolls, which 3086 had all the buffs. This is about 3.09%.
=====  SUITE END  =====
===== SUITE START =====
Name: two desired buffs custom mod usage.
Description: The following tests report how many custom modules were used to get the buffs. None of the buffs are locked during the process.
To get all Attack(10%) and Elemental(10%) for 100000 times:
        2378874 custom moduies were used.
        That is on average 23.78874 modules.
To get all Attack(10%) and MaxAmmo(12%) for 100000 times:
        2047155 custom moduies were used.
        That is on average 20.47155 modules.
To get all ChargeSpeed(12%) and ChargeDamage(12%) for 100000 times:
        1774909 custom moduies were used.
        That is on average 17.74909 modules.
=====  SUITE END  =====
===== SUITE START =====
Name: With locking: two desired buffs custom mod usage.
Description: The following tests report how many custom modules were used to get the buffs. When a desired buff appears, they are immediately locked. The cost of locking a module (2+) is accounted.
To get all Elemental(10%) and Attack(10%) for 100000 times:
        2520202 custom mods were used.
        That is on average 25.20202 modules .
To get all Attack(10%) and MaxAmmo(12%) for 100000 times:
        2334694 custom mods were used.
        That is on average 23.34694 modules .
To get all ChargeSpeed(12%) and ChargeDamage(12%) for 100000 times:
        2154825 custom mods were used.
        That is on average 21.54825 modules .
=====  SUITE END  =====
===== SUITE START =====
Name: Simulate first desired buff locked
Description: The following tests report how many custom modules were used to get the desired buffs. This assumes that the desired buff appeard on the FIRST slot on the first roll and locked immediately. When a desired buff appears, they are immediately locked. The cost of locking a module (2+) is accounted.
With Attack(10%) locked on slot 1 plus getting Elemental(10%) for 100000 times:
        2735448 custom mods were used.
        That is on average 27.35448 modules .
With Attack(10%) locked on slot 1 plus getting MaxAmmo(12%) for 100000 times:
        2377700 custom mods were used.
        That is on average 23.777 modules .
With ChargeDamage(12%) locked on slot 1 plus getting Attack(10%) for 100000 times:
        2681804 custom mods were used.
        That is on average 26.81804 modules .
With ChargeDamage(12%) locked on slot 1 plus getting ChargeSpeed(12%) for 100000 times:
        2354956 custom mods were used.
        That is on average 23.54956 modules .
With Attack(10%) locked on slot 1 plus getting CritDamage(10%) and Elemental(10%) for 100000 times:
        8295826 custom mods were used.
        That is on average 82.95826 modules .
With Attack(10%) locked on slot 1 plus getting MaxAmmo(12%) and Elemental(10%) for 100000 times:
        7546714 custom mods were used.
        That is on average 75.46714 modules .
With Attack(10%) locked on slot 1 plus getting CritDamage(10%) and MaxAmmo(12%) for 100000 times:
        7587975 custom mods were used.
        That is on average 75.87975 modules .
With ChargeDamage(12%) locked on slot 1 plus getting Attack(10%) and Elemental(10%) for 100000 times:
        8073887 custom mods were used.
        That is on average 80.73887 modules .
With ChargeDamage(12%) locked on slot 1 plus getting Attack(10%) and MaxAmmo(12%) for 100000 times:
        7414115 custom mods were used.
        That is on average 74.14115 modules .
With ChargeDamage(12%) locked on slot 1 plus getting MaxAmmo(12%) and ChargeSpeed(12%) for 100000 times:
        6671735 custom mods were used.
        That is on average 66.71735 modules .
=====  SUITE END  =====
===== SUITE START =====
Name: Simulate second desired buff locked
Description: The following tests report how many custom modules were used to get the desired buffs. This assumes that the desired buff appeard on the SECOND slot on the first roll and locked immediately. When a desired buff appears, they are immediately locked. The cost of locking a module (2+) is accounted.
With Attack(10%) locked on slot 2 plus getting Elemental(10%) for 100000 times:
        1945052 custom mods were used.
        That is on average 19.45052 modules .
With Attack(10%) locked on slot 2 plus getting MaxAmmo(12%) for 100000 times:
        1727090 custom mods were used.
        That is on average 17.2709 modules .
With ChargeDamage(12%) locked on slot 2 plus getting Attack(10%) for 100000 times:
        1916956 custom mods were used.
        That is on average 19.16956 modules .
With ChargeDamage(12%) locked on slot 2 plus getting ChargeSpeed(12%) for 100000 times:
        1712632 custom mods were used.
        That is on average 17.12632 modules .
With Attack(10%) locked on slot 2 plus getting Elemental(10%) and CritDamage(10%) for 100000 times:
        7735477 custom mods were used.
        That is on average 77.35477 modules .
With Attack(10%) locked on slot 2 plus getting Elemental(10%) and MaxAmmo(12%) for 100000 times:
        7050546 custom mods were used.
        That is on average 70.50546 modules .
With Attack(10%) locked on slot 2 plus getting CritDamage(10%) and MaxAmmo(12%) for 100000 times:
        7044369 custom mods were used.
        That is on average 70.44369 modules .
With ChargeDamage(12%) locked on slot 2 plus getting Attack(10%) and Elemental(10%) for 100000 times:
        7483010 custom mods were used.
        That is on average 74.8301 modules .
With ChargeDamage(12%) locked on slot 2 plus getting Attack(10%) and MaxAmmo(12%) for 100000 times:
        6900031 custom mods were used.
        That is on average 69.00031 modules .
With ChargeDamage(12%) locked on slot 2 plus getting ChargeSpeed(12%) and MaxAmmo(12%) for 100000 times:
        6232468 custom mods were used.
        That is on average 62.32468 modules .
=====  SUITE END  =====
===== SUITE START =====
Name: Simulate third desired buff locked
Description: The following tests report how many custom modules were used to get the desired buffs. This assumes that the desired buff appeard on the THIRD slot on the first roll and locked immediately. When a desired buff appears, they are immediately locked. The cost of locking a module (2+) is accounted.
With Attack(10%) locked on slot 3 plus getting Elemental(10%) for 100000 times:
        1768350 custom mods were used.
        That is on average 17.6835 modules .
With Attack(10%) locked on slot 3 plus getting MaxAmmo(12%) for 100000 times:
        1579398 custom mods were used.
        That is on average 15.79398 modules .
With ChargeDamage(12%) locked on slot 3 plus getting Attack(10%) for 100000 times:
        1744504 custom mods were used.
        That is on average 17.44504 modules .
With ChargeDamage(12%) locked on slot 3 plus getting ChargeSpeed(12%) for 100000 times:
        1558822 custom mods were used.
        That is on average 15.58822 modules .
With Attack(10%) locked on slot 3 plus getting Elemental(10%) and CritDamage(10%) for 100000 times:
        4963400 custom mods were used.
        That is on average 49.634 modules .
With Attack(10%) locked on slot 3 plus getting MaxAmmo(12%) and Elemental(10%) for 100000 times:
        4560004 custom mods were used.
        That is on average 45.60004 modules .
With Attack(10%) locked on slot 3 plus getting MaxAmmo(12%) and CritDamage(10%) for 100000 times:
        4554684 custom mods were used.
        That is on average 45.54684 modules .
With ChargeDamage(12%) locked on slot 3 plus getting Attack(10%) and Elemental(10%) for 100000 times:
        4871905 custom mods were used.
        That is on average 48.71905 modules .
With ChargeDamage(12%) locked on slot 3 plus getting Attack(10%) and MaxAmmo(12%) for 100000 times:
        4483428 custom mods were used.
        That is on average 44.83428 modules .
With ChargeDamage(12%) locked on slot 3 plus getting ChargeSpeed(12%) and MaxAmmo(12%) for 100000 times:
        4067584 custom mods were used.
        That is on average 40.67584 modules .
=====  SUITE END  =====
```
