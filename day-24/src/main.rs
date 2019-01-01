use pest::*;
use pest_derive::*;
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct UnitGroupParser;

fn parse(input: &str) -> Vec<UnitGroup> {
    let mut unit_groups = Vec::new();
    let mut faction: Option<Faction> = None;
    let mut immune_id = 0;
    let mut infect_id = 0;
    for line in input.lines() {
        if line == "" {
            continue;
        } else if line == "Immune System:" {
            faction = Some(Faction::ImmuneSystem);
            continue;
        } else if line == "Infection:" {
            faction = Some(Faction::Infection);
            continue;
        }

        let pairs = UnitGroupParser::parse(Rule::unit_group, line)
            .unwrap().next().unwrap().into_inner();

        let mut size: Option<Size> = None;
        let mut hit_points: Option<HitPoints> = None;
        let mut weaknesses: HashSet<AttackType> = HashSet::new();
        let mut immunities: HashSet<AttackType> = HashSet::new();
        let mut attack_damage: Option<AttackDamage> = None;
        let mut attack_type: Option<AttackType> = None;
        let mut initiative: Option<Initiative> = None;

        let parse_attack_type = |p: Pair<Rule>| match p.into_inner().next().unwrap().as_rule() {
            Rule::cold        => AttackType::Cold,
            Rule::fire        => AttackType::Fire,
            Rule::radiation   => AttackType::Radiation,
            Rule::slashing    => AttackType::Slashing,
            Rule::bludgeoning => AttackType::Bludgeoning,
            unknown           => panic!(format!("{:?}", unknown)),
        };
        for p in pairs {
            match p.as_rule() {
                Rule::size => size = p.as_str().parse::<Size>().ok(),
                Rule::hit_points => hit_points = p.as_str().parse::<HitPoints>().ok(),
                Rule::attack_damage => attack_damage = p.as_str().parse::<AttackDamage>().ok(),
                Rule::initiative => initiative = p.as_str().parse::<Initiative>().ok(),
                Rule::attack_type => attack_type = Some(parse_attack_type(p)),
                Rule::weaknesses => weaknesses.extend(p.into_inner().map(parse_attack_type)),
                Rule::immunities => immunities.extend(p.into_inner().map(parse_attack_type)),
                unknown => panic!(format!("{:?}", unknown)),
            }
        }

        let id = match faction {
            Some(Faction::ImmuneSystem) => {
                immune_id += 1;
                Some(immune_id)
            }
            Some(Faction::Infection) => {
                infect_id += 1;
                Some(infect_id)
            }
            None => None
        };
        let unit_group = UnitGroup {
            id: id.unwrap(),
            target: None,
            is_targetted: false,
            size: size.unwrap(),
            hit_points: hit_points.unwrap(),
            attack_damage: attack_damage.unwrap(),
            attack_type: attack_type.unwrap(),
            initiative: initiative.unwrap() - 1, // 0-index
            weaknesses: weaknesses,
            immunities: immunities,
            faction: faction.unwrap()
        };
        unit_groups.push(unit_group);
    }
    unit_groups
}

#[derive(Debug,Hash,Ord,PartialOrd,Eq,PartialEq,Clone)]
enum AttackType {
    Cold,
    Fire,
    Radiation,
    Slashing,
    Bludgeoning,
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Faction {
    ImmuneSystem,
    Infection,
}

type Id           = usize;
type Size         = usize;
type HitPoints    = usize;
type AttackDamage = usize;
type Initiative   = usize;

#[derive(Debug,Clone)]
struct UnitGroup {
    id: Id,
    target: Option<Initiative>,
    is_targetted: bool,
    size: Size,
    hit_points: HitPoints,
    attack_damage: AttackDamage,
    attack_type: AttackType,
    initiative: Initiative,
    weaknesses: HashSet<AttackType>,
    immunities: HashSet<AttackType>,
    faction: Faction,
}

impl UnitGroup {
    fn effective_power(&self) -> AttackDamage {
        self.size * self.attack_damage
    }
    fn immune_to(&self, other: &UnitGroup) -> bool {
        self.immunities.contains(&other.attack_type)
    }
    fn weak_to(&self, other: &UnitGroup) -> bool {
        self.weaknesses.contains(&other.attack_type)
    }
    fn take_damage(&mut self, damage: AttackDamage) {
        let remaining_hit_points = (self.size * self.hit_points).saturating_sub(damage);
        if remaining_hit_points > 0 {
            self.size = (remaining_hit_points / self.hit_points) + 1;
        } else {
            self.size = 0;
        }
    }
}

use std::fmt;

impl fmt::Display for UnitGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f,
            "Size: {}, Hp: {}, Dmg: {}, Type: {:?}, Immune: {:?}, Weak: {:?}",
            self.size, self.hit_points, self.attack_damage,
            self.attack_type, self.immunities, self.weaknesses
        );
        Ok(())
    }
}



fn print<'a, A, B>(immune: A, infect: B)
where
    A: Iterator<Item = &'a UnitGroup>,
    B: Iterator<Item = &'a UnitGroup>,
{
    println!("");
    println!("Immune System:");
    for u in immune {
        if u.size > 0 {
            println!("Group {} ({}) contains {} units, {}", u.id, u.initiative, u.size, u)
        }
    }
    println!("");
    println!("Infection:");
    for u in infect {
        if u.size > 0 {
            println!("Group {} ({}) contains {} units, {}", u.id, u.initiative, u.size, u);
        }
    }
    println!("");
}

use std::cmp::Reverse;
use std::cmp::Ordering;

fn fight(unit_groups: Vec<UnitGroup>) -> (Faction,Size) {
    // Target selection
    let mut phase_1: Vec<UnitGroup> = unit_groups;
    let mut phase_2: Vec<UnitGroup> = Vec::new();
    let num_unit_groups = phase_1.len();
    let mut prev_size: Size = phase_1.iter().map(|u| u.size).sum();

    loop {
        phase_1.sort_by_key(|u| Reverse(u.initiative));
        phase_1.sort_by_key(|u| Reverse(u.effective_power()));
        phase_1.reverse();

        let mut is_targetted = vec![false; num_unit_groups];

        while let Some(mut attacker) = phase_1.pop() {
            attacker.target = None;
            if attacker.size == 0 {
                phase_2.push(attacker);
                continue;
            }
            attacker.target = phase_1.iter().chain(phase_2.iter())
                .filter(|u| u.faction != attacker.faction)
                .filter(|u| u.size > 0)
                .filter(|u| !is_targetted[u.initiative])
                .filter(|u| !u.immune_to(&attacker))
                .max_by(|a,b|
                    match (a.weak_to(&attacker), b.weak_to(&attacker)) {
                        (true,false) => Ordering::Greater,
                        (false,true) => Ordering::Less,
                        _ => match (a.effective_power(), b.effective_power()) {
                            (x,y) if x < y => Ordering::Less,
                            (x,y) if x > y => Ordering::Greater,
                            _ => match (a.initiative, b.initiative) {
                                (x,y) if x < y => Ordering::Less,
                                (x,y) if x > y => Ordering::Greater,
                                _              => Ordering::Equal,
                            }
                        }
                    }
                )
                .map(|defender| {
                    is_targetted[defender.initiative] = true;
                    defender.initiative
                });
            phase_2.push(attacker);
        }

        // Attacking
        phase_2.sort_by_key(|u| Reverse(u.initiative));

        for attacker in 0..num_unit_groups {
            if phase_2[attacker].size > 0 {
                if let Some(defender) = phase_2[attacker].target {
                    let defender = num_unit_groups-defender-1;
                    if phase_2[defender].size == 0 {
                        continue;
                    }
                    let damage = phase_2[attacker].effective_power();
                    if !phase_2[defender].immune_to(&phase_2[attacker]) {
                        if phase_2[defender].weak_to(&phase_2[attacker]) {
                            phase_2[defender].take_damage(damage * 2);
                        } else {
                            phase_2[defender].take_damage(damage);
                        }
                    }
                }
            }
        }

        let immune_size: Size = phase_2
            .iter()
            .filter(|u| u.faction == Faction::ImmuneSystem)
            .map(|u| u.size)
            .sum();
        let infect_size: Size = phase_2
            .iter()
            .filter(|u| u.faction == Faction::Infection)
            .map(|u| u.size)
            .sum();

        if immune_size == 0 {
            return (Faction::Infection, infect_size);
        } else if infect_size == 0 {
            return (Faction::ImmuneSystem, immune_size);
        } else if infect_size+immune_size == prev_size {
            if immune_size > infect_size {
                return (Faction::ImmuneSystem, immune_size)
            } else {
                return (Faction::Infection, infect_size)
            }
        } else {
            prev_size = infect_size+immune_size;
            phase_1 = phase_2;
            phase_2 = Vec::new();
        }
    }
}

fn binary_search(unit_groups: &Vec<UnitGroup>, min: usize, max: usize) {
    let mid = (max - min)/2 + min;
    if min == mid || max == mid {
        return;
    }
    let mut boosted = unit_groups.clone();
    boosted
        .iter_mut()
        .filter(|u| u.faction == Faction::ImmuneSystem)
        .for_each(|u| u.attack_damage += mid);
    match fight(boosted) {
        (Faction::ImmuneSystem, remaining) => {
            println!(":) {} => {}", mid, remaining);
            binary_search(unit_groups, min, mid)
        }
        (Faction::Infection, remaining) => {
            println!(":( {} => {}", mid, remaining);
            binary_search(unit_groups, mid, max)
        }
    }
}

fn main() {
    let unit_groups = parse(include_str!("input"));
    binary_search(&unit_groups, 0, 190000);
    println!("{:?}", fight(unit_groups));
    //println!("");
    //println!("{:?}", immune);
    //println!("");
    //println!("{:?}", infect);
}

#[test]
fn test1() {
    parse(include_str!("example_input"));
}
