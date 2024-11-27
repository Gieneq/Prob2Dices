use rand::prelude::*;


fn float_eq(a: f32, b: f32) -> bool {
    const EPSILON: f32 = 0.0001;
    (a - b).abs() < EPSILON
}

fn float_lt(a: f32, b: f32) -> bool {
    if float_eq(a, b) {
        return false;
    }
    a < b
}

fn calc_2dices_value_probability(value: u16) -> f32 {
    match value {
        2 | 12 => { 1f32 / 36f32 },
        3 | 11 => { 2f32 / 36f32 },
        4 | 10 => { 3f32 / 36f32 },
        5 | 9 => { 4f32 / 36f32 },
        6 | 8 => { 5f32 / 36f32 },
        7 => { 6f32 / 36f32 },
        _ => { 0f32 },
    }
}

fn gen_2dices_random_values() -> Vec<u16> {
    let mut nums: Vec<u16> = (2..=12).collect();
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
    nums
}

fn calculate_probability_of_2dices_random_values(values: &[u16]) -> f32 {
    values.iter().fold(0f32, |acc, x| acc + calc_2dices_value_probability(*x))
}

fn find_nums_coverage(target_probabilities: &[f32], nums: &[u16]) -> Option<Vec<Vec<u16>>> {
    let mut result: Vec<Vec<u16>> = vec![];

    let mut tmp_result: Vec<u16> = vec![];

    let mut target_prob_iter = target_probabilities.iter();

    let mut target_probability = match target_prob_iter.next() {
        Some(&v) => { v },
        None => { return None; },
    };

    let mut recent_probability = 0f32;

    for &num in nums {
        tmp_result.push(num);
        recent_probability += calc_2dices_value_probability(num);

        if recent_probability >= target_probability {
            tmp_result.sort();
            result.push(tmp_result.clone());
            target_probability = match target_prob_iter.next() {
                Some(&v) => v,
                None => break,
            };
        }
    }

    if result.len() != target_probabilities.len() {
        return None;
    }

    Some(result)
}

pub fn measure_coverage_deviation(target_probabilities: &[f32], coverage: &[Vec<u16>]) -> f32 {
    target_probabilities
        .iter()
        .zip(coverage.iter())
        .map(|(&target_prob, coverage)| {
            let tmp_probability = calculate_probability_of_2dices_random_values(coverage.as_slice());
            (target_prob - tmp_probability).powi(2)
        })
        .sum::<f32>().sqrt()
}

pub fn find_best_coverage(target_probabilities: &[f32]) -> Option<Vec<Vec<u16>>> {
    const ROLLS_COUNT: usize = 100;
    match target_probabilities.len() {
        0 => { Some(vec![gen_2dices_random_values()]) },
        _ => {
            let mut recent_best_match: Option<Vec<Vec<u16>>> = None;
            let mut recent_deviation = 10000f32;

            for _ in 0..ROLLS_COUNT {
                let recent_nums = gen_2dices_random_values();
                match find_nums_coverage(target_probabilities, &recent_nums) {
                    Some(coverage) => {
                        let deviation = measure_coverage_deviation(target_probabilities, &coverage);

                        if recent_best_match.is_none() || float_lt(deviation, recent_deviation) {
                            #[cfg(debug_assertions)]
                            {
                                print_coverage(&coverage, target_probabilities, deviation);
                            }
                            recent_best_match = Some(coverage);
                            recent_deviation = deviation;
                        }
                    },
                    None => continue
                }

            }

            recent_best_match
        },
    }
}

pub fn print_coverage(coverage: &[Vec<u16>], target_probabilities: &[f32], deviation: f32) {
    println!("Coverage found with deviation: {}:", deviation);
    for (i, group) in coverage.iter().enumerate() {
        println!("Group {}: {:?}, prob: {}, target: {}", i + 1, 
            group, 
            calculate_probability_of_2dices_random_values(group.as_slice()),
            target_probabilities[i]);
    }
}



#[test]
fn test_probabilities() {
    assert_eq!(calc_2dices_value_probability(2), 1f32/36f32);

    let nums = gen_2dices_random_values();
    assert_eq!(nums.len(), 11);

    let nums_sum = nums.iter().sum::<u16>();
    let probability = calculate_probability_of_2dices_random_values(&nums);

    assert_eq!(nums_sum, 77);
    assert!(float_eq(probability, 1f32));
}

#[test]
fn test_matching_probabilities() {
    let target_probabilities = [0.1, 0.3, 0.6];
    let nums: [u16; 10] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    let coverage_option = find_nums_coverage(&target_probabilities, &nums);

    assert!(coverage_option.is_some());
    let coverage = coverage_option.unwrap();

    assert_eq!(coverage.len(), target_probabilities.len());

    assert_eq!(coverage[0], [2, 3, 4]);
    assert_eq!(coverage[1], [2, 3, 4, 5, 6]);
    assert_eq!(coverage[2], [2, 3, 4, 5, 6, 7, 8]);

    assert!(calculate_probability_of_2dices_random_values(&coverage[0]) >= target_probabilities[0]);
    assert!(calculate_probability_of_2dices_random_values(&coverage[1]) >= target_probabilities[1]);
    assert!(calculate_probability_of_2dices_random_values(&coverage[2]) >= target_probabilities[2]);

    let deviation = measure_coverage_deviation(&target_probabilities, &coverage);

    print_coverage(&coverage, &target_probabilities, deviation);
}