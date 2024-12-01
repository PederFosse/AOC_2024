use std::fs::read_to_string;

fn get_total_distances(mut vec1: Vec<u32>, mut vec2: Vec<u32>) -> u32 {
    vec1.sort();
    vec2.sort();

    let mut sum = 0;

    while let (Some(ls), Some(rs)) = (vec1.pop(), vec2.pop()) {
        let abs_diff = rs.abs_diff(ls);
        sum += abs_diff;
    }

    sum
}

fn calculate_similarity_score(vec1: Vec<u32>, vec2: Vec<u32>) -> u32 {
    let mut similarity_score = 0;
    for num in vec1 {
        let count = vec2.iter().filter(|&&x| x == num).count();
        similarity_score += count as u32 * num;
    }

    similarity_score
}

pub fn main() {
    let contents = read_to_string("src/data/day01/dummy.txt").expect("Unreadable file");
    let lines = contents.split("\n");

    let mut vec1: Vec<u32> = Vec::new();
    let mut vec2: Vec<u32> = Vec::new();

    for ln in lines {
        let mut numbers = ln.split_whitespace();
        let Some(number1) = numbers.next() else {
            break;
        };

        let Some(number2) = numbers.next() else {
            break;
        };

        vec1.push(number1.parse().expect("Could not parse number"));
        vec2.push(number2.parse().expect("Could not parse number"));
    }

    let sum = get_total_distances(vec1.clone(), vec2.clone());
    let similarity_score = calculate_similarity_score(vec1, vec2);

    println!(
        "Total distance: {}\nSimilarity score: {}",
        sum, similarity_score
    )
}
