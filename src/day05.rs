use std::fs::read_to_string;

fn prepare_puzzle_input(input: &str) -> (&str, &str) {
    let mut split = input.split("\n\n");

    (split.next().unwrap().trim(), split.next().unwrap().trim())
}

struct Rule {
    first: u8,
    second: u8,
}

impl Rule {
    fn new(input: &str) -> Self {
        let mut split = input.split("|");
        Rule {
            first: split.next().unwrap().parse().unwrap(),
            second: split.next().unwrap().parse().unwrap(),
        }
    }

    /**
     * Applies the rule and returns true if valid
     */
    fn apply(&self, page: &Vec<u8>) -> bool {
        if !page.contains(&self.first) || !page.contains(&self.second) {
            return true;
        }
        page.iter().position(|el| el == &self.first) < page.iter().position(|el| el == &self.second)
    }

    fn apply_and_fix(&self, mut page: Vec<u8>) -> (bool, Vec<u8>) {
        if self.apply(&page) == false {
            let pos1 = page.iter().position(|el| el == &self.first).unwrap();
            let pos2 = page.iter().position(|el| el == &self.second).unwrap();

            if pos1 > pos2 {
                // Swap position between the items
                let item1 = page[pos1];
                let item2 = page[pos2];
                page[pos1] = item2;
                page[pos2] = item1;
            }
            return (true, page);
        }

        (false, page)
    }
}

struct RuleCollection {
    rules: Vec<Rule>,
}

impl RuleCollection {
    fn new(input: &str) -> Self {
        let rules = input.split("\n").map(|line| Rule::new(line));
        RuleCollection {
            rules: rules.collect(),
        }
    }

    fn apply_to_pages(&self, pages: &str) -> (u32, u32) {
        let mut sum: u32 = 0;

        let mut invalid_pages: Vec<Vec<u8>> = vec![];

        'page: for page in pages.split("\n") {
            let page_as_vec = page
                .split(",")
                .map(|el| el.parse::<u8>().unwrap())
                .collect();

            for rule in &self.rules {
                if !rule.apply(&page_as_vec) {
                    invalid_pages.push(page_as_vec);
                    continue 'page;
                }
            }

            // All rules applied and valid
            let page_in_middle = page_as_vec[page_as_vec.len() / 2];
            sum += page_in_middle as u32
        }

        let mut part_2_sum = 0;
        for mut invalid_page in invalid_pages {
            // Swap 'til everything's right
            loop {
                let mut swaps = 0;
                for rule in &self.rules {
                    let (did_swap, result) = rule.apply_and_fix(invalid_page.clone());
                    invalid_page = result;
                    if did_swap == true {
                        swaps += 1
                    }
                }
                if swaps == 0 {
                    break;
                }
            }

            // All rules applied and valid
            let page_in_middle = invalid_page[invalid_page.len() / 2];
            part_2_sum += page_in_middle as u32
        }

        (sum, part_2_sum)
    }
}

pub fn main() {
    let puzzle_input = read_to_string("src/data/day05/input.txt").expect("Failed to open file");
    let (rules, pages) = prepare_puzzle_input(&puzzle_input);

    let rule_collection = RuleCollection::new(rules);

    let (part1, part2) = rule_collection.apply_to_pages(pages);

    println!("Part 1: {}, Part 2: {}", part1, part2);
}
