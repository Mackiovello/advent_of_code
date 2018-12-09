use self::part_one::find_order;
use self::step::Step;

use super::get_input_vec;

use regex::Regex;
use std::collections::HashSet;

mod part_one;
mod step;

// ACHOQRXSEKUGMYIWDZLNBFTJVP - correct
pub fn problem_seven_part_one() -> String {
    let input = get_input_vec("seven.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    find_order(&input_refs).unwrap()
}

pub fn problem_seven_part_two() -> u32 {
    let input = get_input_vec("seven.txt");
    let input_refs = input.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    time_to_complete(&input_refs, 5, 60)
}

fn time_to_complete(steps: &[&str], workers: u32, base_time: u32) -> u32 {
    let parsed_steps: Vec<(&str, &str)> = parse_input(steps);
    let char_steps = to_char_steps(&parsed_steps);
    // let constructed_steps = construct_steps(&char_steps);
    0
    // execute_steps(constructed_steps, workers, base_time).unwrap()
}

fn order_for_steps(steps: &[&str]) -> Vec<char> {
    find_order(steps).unwrap().chars().collect()
}

fn construct_steps2<'a>() -> Vec<Step<'a>> {
    let steps: Vec<Step> = vec!['A', 'B', 'C'].iter().map(|&u| Step::new(u)).collect();
    let step: &Step = steps.clone().get(3).unwrap();
    steps[0].add_dependency(step);
    steps
}
// fn construct_steps(dependencies: &[(char, char)]) -> Vec<Step> {
//     let unique_letters = to_unique_letters(dependencies);

//     let steps: Vec<Step> = vec!['A', 'B', 'C'].iter().map(|&u| Step::new(u)).collect();

//     for i in 0..(steps.len() - 1) {
//         let representation = steps[i].get_representation();

//         let dependency_representation: Vec<char> = dependencies
//             .iter()
//             .filter(|d| d.1 == representation)
//             .map(|d| d.0)
//             .collect();

//         let dependencies: Vec<&Step> = steps
//             .iter()
//             .filter(|d| dependency_representation.contains(&&d.get_representation()))
//             .collect();

//         for dependency in dependencies {
//             steps[i].add_dependency(dependency);
//         }
//     }

//     steps
// }
// "Step A must be finished before step D can begin.",

fn to_char_steps(steps: &[(&str, &str)]) -> Vec<(char, char)> {
    steps
        .iter()
        .map(|s| {
            (
                s.0.chars().collect::<Vec<_>>()[0],
                s.1.chars().collect::<Vec<_>>()[0],
            )
        })
        .collect()
}

fn to_unique_letters<'a>(steps: &[(char, char)]) -> HashSet<char> {
    steps.iter().map(|s| vec![s.0, s.1]).flatten().collect()
}

fn execute_steps(steps: Vec<Step>, workers: u32, base_time: u32) -> Result<u32, String> {
    if workers == 0 {
        return Err("Can't execute steps with no workers.".to_owned());
    }

    if steps.is_empty() {
        return Ok(0);
    }

    let number_of_steps = steps.len();

    if number_of_steps == 1 {
        return Ok(steps[0].get_time_to_finish(base_time));
    }

    // if steps.len() <= workers as usize && steps.iter().all(|s| s.can_be_finished()) {
    //     return Ok(steps
    //         .iter()
    //         .max_by(|a, b| {
    //             a.get_time_to_finish(base_time)
    //                 .cmp(&b.get_time_to_finish(base_time))
    //         })
    //         .unwrap()
    //         .get_time_to_finish(base_time));
    // }

    Ok(0)
}

fn get_finished_steps(steps: &[Step]) -> Vec<char> {
    steps
        .iter()
        .filter(|s| s.is_finished())
        .map(|s| s.get_representation())
        .collect()
}

fn parse_input<'a>(steps: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let reg =
        Regex::new(r"^Step (?P<first>\w) must be finished before step (?P<second>\w) can begin.$")
            .unwrap();

    steps
        .iter()
        .map(|s| {
            let captured = reg.captures(s).unwrap();
            (
                captured.name("first").unwrap(),
                captured.name("second").unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_workers_errs() {
        // Given
        let workers = 0;
        let base_time = 60;
        let steps = (68u8..72).map(|n| Step::new(n as char)).collect();

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(
            Err("Can't execute steps with no workers.".to_owned()),
            result
        );
    }

    #[test]
    fn no_steps_return_0() {
        // Given
        let workers = 3;
        let base_time = 60;
        let steps = vec![];

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(0), result);
    }

    #[test]
    fn one_step_returns_time_to_finish_step() {
        // Given
        let workers = 3;
        let base_time = 60;
        let steps = vec![Step::new('A')];
        let expected_to_finish = steps[0].get_time_to_finish(base_time);

        // When
        let result = execute_steps(steps, workers, base_time);

        // Then
        assert_eq!(Ok(expected_to_finish), result);
    }

    // #[test]
    // fn two_steps_without_dependencies_return_time_for_longest_with_two_workers() {
    //     // Given
    //     let workers = 2;
    //     let base_time = 60;
    //     let steps = vec![Step::new('A'), Step::new('B')];
    //     let expected_to_finish = steps[1].get_time_to_finish(base_time);

    //     // When
    //     let result = execute_steps(steps, workers, base_time);

    //     // Then
    //     assert_eq!(Ok(expected_to_finish), result);
    // }

    #[test]
    fn two_steps_dependent_on_eachother_returns_combined_time() {
        // // Given
        // let workers = 2;
        // let base_time = 60;
        // let step_a = Step::new('A', vec![]);
        // let step_b = Step::new('B', vec![&step_a]);
        // // let steps = vec![step_b, step_a];

        // // When
        // let result = execute_steps(steps, workers, base_time);

        // // Then
        // assert_eq!(Ok(0), result);
    }

    #[test]
    fn sample_part_2() {
        // Given
        let input = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];
        // When
        let result = time_to_complete(&input, 2, 0);

        // Then
        assert_eq!(15, result);
    }
}
