#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut file = File::open("inputs/day_14.txt").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let receipts = parse_data(&data);

    println!("Task A: {}", task_A(&receipts));
    println!("Task B: {}", task_B(&receipts, 1000000000000));
}

struct Ingredient {
    name: String,
    amount: i64,
}

struct Reaction {
    raw: Vec<Ingredient>,
    result: Ingredient,
}

fn parse_ingredient(raw: &str) -> Ingredient {
    let data: Vec<&str> = raw.trim().split(' ').map(|x| x.trim()).collect();
    Ingredient { name: data[1].to_owned(),
               amount: data[0].parse().unwrap() }
}

fn parse_data(data: &str) -> HashMap<String, Reaction> {
    let mut receipts: HashMap<String, Reaction> = HashMap::new();
    let lines: Vec<&str> = data.split('\n').collect();
    for line in lines {
        let halves: Vec<&str> = line.split('=').collect();
        let ings: Vec<Ingredient> = halves[0].split(',').map(|x| parse_ingredient(x)).collect();
        let result = parse_ingredient(&halves[1][2..]);
        receipts.insert(result.name.clone(), Reaction{ raw: ings, result });
    }
    receipts
}

fn TopSort(data: &HashMap<String, Reaction>) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for r in data.values() {
        for ingr in &r.raw {
            graph.entry(ingr.name.clone()).or_insert(Vec::new()).push(r.result.name.clone());
        }
    }
    for r in data.keys() {
        graph.entry(r.clone()).or_insert(Vec::new());
    }

    let mut sorted: Vec<String> = Vec::new();
    while !graph.is_empty() {
        let mut next: String = String::new();
        for candidate in graph.keys() {
            let mut dependent = false;
            for g in graph.values() {
                if g.contains(candidate) {
                    dependent = true;
                    break;
                }
            }
            if dependent { continue; }
            next = candidate.clone();
            break;
        }
        assert!(!next.is_empty());
        graph.remove(&next);
        sorted.push(next);
    }
    sorted.reverse();
    sorted
}

fn ore_for_fuel(receipts: &HashMap<String, Reaction>,
                needs: &mut HashMap<String, i64>,
                sorted: &[String]) {

    for x in sorted {
        if !needs.contains_key(x) { continue; }
        if x == "ORE" { break; }
        if needs[x] < 0 { continue; }

        let r = &receipts[x];
        let coeff = (needs[x] - 1) / r.result.amount + 1;
        for ingr in r.raw.iter() {
            *needs.entry(ingr.name.clone()).or_insert(0) += coeff * ingr.amount;
        }
        *needs.get_mut(x).unwrap() -= r.result.amount * coeff;
    }
}


fn task_A(receipts: &HashMap<String, Reaction>) -> i64 {
    let sorted = TopSort(receipts);
    let mut needs: HashMap<String, i64> = HashMap::new();
    needs.insert("FUEL".to_string(), 1);
    ore_for_fuel(receipts, &mut needs, &sorted);
    needs["ORE"]
}

fn task_B(receipts: &HashMap<String, Reaction>, stock: i64) -> i64 {
    let sorted = TopSort(receipts);
    let mut needs: HashMap<String, i64> = HashMap::new();
    needs.insert("ORE".to_string(), -stock);
    needs.insert("FUEL".to_string(), 0);

    let mut fuel = 0;
    let mut attempt = 1;

    while attempt > 0 {
        let needs_backup = needs.clone();
        *needs.get_mut("FUEL").unwrap() = attempt;
        ore_for_fuel(receipts, &mut needs, &sorted);
        if needs["ORE"] > 0 {
            needs = needs_backup;
            attempt /= 2;
            continue;
        }

        fuel += attempt - needs["FUEL"];
        attempt *= 2;
    }
    fuel
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_A() {
        let rs = parse_data("10 ORE => 10 A
                            1 ORE => 1 B
                            7 A, 1 B => 1 C
                            7 A, 1 C => 1 D
                            7 A, 1 D => 1 E
                            7 A, 1 E => 1 FUEL");
        assert_eq!(task_A(&rs), 31);

        let rs = parse_data("9 ORE => 2 A
                            8 ORE => 3 B
                            7 ORE => 5 C
                            3 A, 4 B => 1 AB
                            5 B, 7 C => 1 BC
                            4 C, 1 A => 1 CA
                            2 AB, 3 BC, 4 CA => 1 FUEL");
        assert_eq!(task_A(&rs), 165);

        let rs = parse_data("157 ORE => 5 NZVS
                            165 ORE => 6 DCFZ
                            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                            179 ORE => 7 PSHF
                            177 ORE => 5 HKGWZ
                            7 DCFZ, 7 PSHF => 2 XJWVT
                            165 ORE => 2 GPVTF
                            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
        assert_eq!(task_A(&rs), 13312);

        let rs = parse_data("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                            17 NVRVD, 3 JNWZP => 8 VPVL
                            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                            22 VJHF, 37 MNCFX => 5 FWMGM
                            139 ORE => 4 NVRVD
                            144 ORE => 7 JNWZP
                            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                            145 ORE => 6 MNCFX
                            1 NVRVD => 8 CXFTF
                            1 VJHF, 6 MNCFX => 4 RFSQX
                            176 ORE => 6 VJHF");
        assert_eq!(task_A(&rs), 180697);

        let rs = parse_data("171 ORE => 8 CNZTR
                            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                            114 ORE => 4 BHXH
                            14 VRPVC => 6 BMBT
                            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                            5 BMBT => 4 WPTQ
                            189 ORE => 9 KTJDG
                            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                            12 VRPVC, 27 CNZTR => 2 XDBXC
                            15 KTJDG, 12 BHXH => 5 XCVML
                            3 BHXH, 2 VRPVC => 7 MZWV
                            121 ORE => 7 VRPVC
                            7 XCVML => 6 RJRHP
                            5 BHXH, 4 VRPVC => 5 LTCX");
        assert_eq!(task_A(&rs), 2210736);
    }

    #[test]
    fn test_B() {
        let stock = 1000000000000;

        let rs = parse_data("157 ORE => 5 NZVS
                            165 ORE => 6 DCFZ
                            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                            179 ORE => 7 PSHF
                            177 ORE => 5 HKGWZ
                            7 DCFZ, 7 PSHF => 2 XJWVT
                            165 ORE => 2 GPVTF
                            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT");
        assert_eq!(task_B(&rs, stock), 82892753);

        let rs = parse_data("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                            17 NVRVD, 3 JNWZP => 8 VPVL
                            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                            22 VJHF, 37 MNCFX => 5 FWMGM
                            139 ORE => 4 NVRVD
                            144 ORE => 7 JNWZP
                            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                            145 ORE => 6 MNCFX
                            1 NVRVD => 8 CXFTF
                            1 VJHF, 6 MNCFX => 4 RFSQX
                            176 ORE => 6 VJHF");
        assert_eq!(task_B(&rs, stock), 5586022);

        let rs = parse_data("171 ORE => 8 CNZTR
                            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                            114 ORE => 4 BHXH
                            14 VRPVC => 6 BMBT
                            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                            5 BMBT => 4 WPTQ
                            189 ORE => 9 KTJDG
                            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                            12 VRPVC, 27 CNZTR => 2 XDBXC
                            15 KTJDG, 12 BHXH => 5 XCVML
                            3 BHXH, 2 VRPVC => 7 MZWV
                            121 ORE => 7 VRPVC
                            7 XCVML => 6 RJRHP
                            5 BHXH, 4 VRPVC => 5 LTCX");
        assert_eq!(task_B(&rs, stock), 460664);
    }
}
