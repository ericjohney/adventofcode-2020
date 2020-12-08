use adventofcode2020::file_utils;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let associations = file_utils::lines("inputs/day7.txt");

    part1(&associations);
    part2(&associations);
}

fn part1(associations: &Vec<String>) {
    let mut parents = HashMap::new();
    for association in associations {
        let s = association.replace(" bags", "").replace(" bag", "");
        let bags: Vec<_> = s.split(" contain ").collect();
        let parent = bags[0].trim();
        let children: HashSet<_> = bags[1]
            .trim_matches('.')
            .split(", ")
            .map(|bag| bag.trim_matches(char::is_numeric).trim())
            .collect();
        for child in children {
            let p = parents.entry(child.to_string()).or_insert(HashSet::new());
            if child != "no other" {
                p.insert(parent.to_string());
            }
        }
    }

    fn get_parents(parents: &HashMap<String, HashSet<String>>, color: &str) -> HashSet<String> {
        let mut results = HashSet::new();
        if let Some(p) = parents.get(color) {
            if p.len() > 0 {
                results.extend(p.iter().cloned());
                p.iter()
                    .map(|c| get_parents(parents, c))
                    .for_each(|r| results.extend(r.iter().cloned()));
            }
        }
        results
    };

    println!(
        "part 1 colors {}",
        get_parents(&parents, "shiny gold").len()
    );
}

fn part2(associations: &Vec<String>) {
    let mut bag_map = HashMap::new();
    for association in associations {
        let s = association.replace(" bags", "").replace(" bag", "");
        let bags: Vec<_> = s.split(" contain ").collect();
        let parent = bags[0].trim();
        let children = Regex::new(r"(\d+) (.*?)(?:,|\.)")
            .unwrap()
            .captures_iter(&bags[1])
            .map(|c| {
                let count = str::parse::<i64>(&c[1]).unwrap();
                (c[2].to_string(), count)
            })
            .collect::<HashMap<_, _>>();
        bag_map.insert(parent.to_string(), children);
    }

    fn count_children(bag_map: &HashMap<String, HashMap<String, i64>>, color: &str) -> i64 {
        let mut count = 0;
        if let Some(p) = bag_map.get(color) {
            if p.len() > 0 {
                for (c, subcount) in p {
                    count += subcount;
                    for _ in 0..(*subcount as usize) {
                        count += count_children(bag_map, c);
                    }
                }
            }
        }
        count
    };

    println!("part 1 colors {}", count_children(&bag_map, "shiny gold"));
}
