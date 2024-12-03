fn is_safe(nums: &[usize]) -> usize {
    // If they're not increasing or decreasing, then do an early return.
    if nums[0] == nums[1] {
        return 0;
    }
    // Determine the direction.
    let increasing = nums[0] < nums[1];
    // For each window, check if it meets the requirements.
    for window in nums.windows(2) {
        let (a, b) = (window[0], window[1]);
        if ((a < b) != increasing) || (a.abs_diff(b) < 1) || (a.abs_diff(b) > 3) {
            return 0;
        }
    }
    1
}

fn part1(lines: impl Iterator<Item = String>) -> String {
    lines
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("We know AoC input is good"))
                .collect::<Vec<usize>>()
        })
        .map(|nums| is_safe(&nums))
        .sum::<usize>()
        .to_string()
}

// This doesn't work :( Instead, I just brute-force it.

// fn part2(lines: impl Iterator<Item = String>) -> String {
//     lines
//         .map(|line| {
//             line.split_whitespace()
//                 .map(|s| s.parse().expect("We know AoC input is good"))
//                 .collect::<Vec<usize>>()
//         })
//         .map(|nums| {
//             // If more than one of the first five numbers aren't increasing or decreasing, do an
//             // early return.
//             let first_four_equal: [bool; 4] = [
//                 nums[0] == nums[1],
//                 nums[1] == nums[2],
//                 nums[2] == nums[3],
//                 nums[3] == nums[4],
//             ];
//             if first_four_equal.iter().filter(|b| **b == true).count() > 1 {
//                 println!("Unsafe (more than one equal): {:?}", nums);
//                 return 0;
//             }
//             // Determine the direction by checking the first four directions. We are lucky that our
//             // input is always at least 5 numbers long.
//             let first_four_directions: [bool; 4] = [
//                 nums[0] < nums[1],
//                 nums[1] < nums[2],
//                 nums[2] < nums[3],
//                 nums[3] < nums[4],
//             ];
//             // If we have different directions two times, then we know we need to remove more than
//             // one unsafe level, which we aren't allowed to do.
//             if first_four_directions
//                 .iter()
//                 .filter(|b| **b == false)
//                 .count()
//                 == 2
//             {
//                 println!("Unsafe (different directions): {:?}", nums);
//                 return 0;
//             }
//             // Finally, our direction will be the most common one.
//             let increasing: bool = first_four_directions
//                 .into_iter()
//                 .max()
//                 .expect("We know we have good data here.");
//
//             // Keep track of whether we've already skipped.
//             let mut skipped_before = false;
//             // If we've just skipped, the lower index will be one lower than usual.
//             let mut low_i: usize = 0;
//
//             for i in 1..nums.len() {
//                 let i_valid = ((nums[low_i] < nums[i]) == increasing)
//                     && (nums[low_i].abs_diff(nums[i]) >= 1)
//                     && (nums[low_i].abs_diff(nums[i]) <= 3);
//                 if i_valid {
//                     // Increment the low_i as usual.
//                     low_i = i;
//                     continue;
//                 }
//                 // If we've already skipped one, we have two or more unsafe levels, so return 0.
//                 if skipped_before {
//                     println!("Unsafe (skipped before): {:?}", nums);
//                     return 0;
//                 }
//                 // If we are at the end of the list and haven't skipped before, we can skip the last
//                 // one and thus make the reading safe!
//                 if i == nums.len() - 1 {
//                     return 1;
//                 }
//                 // With that check, we know that i + 1 will be in bounds for the nums vec.
//                 if ((nums[low_i] < nums[i + 1]) == increasing)
//                     && (nums[low_i].abs_diff(nums[i + 1]) >= 1)
//                     && (nums[low_i].abs_diff(nums[i + 1]) <= 3)
//                 {
//                     // If skipping will yield a safe level, then we skip.
//                     skipped_before = true;
//                     // Change the low i so we actually skip in the comparisons.
//                     low_i = i - 1;
//                 } else {
//                     // Otherwise, we'll return 0 because skipping doesn't help.
//                     println!("Unsafe (skipping doesn't help): {:?}", nums);
//                     return 0;
//                 }
//             }
//             return 1;
//         })
//         .sum::<usize>()
//         .to_string()
// }

fn part2(lines: impl Iterator<Item = String>) -> String {
    lines
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("We know AoC input is good"))
                .collect::<Vec<usize>>()
        })
        .map(|nums| {
            let mut options: Vec<Vec<usize>> = Vec::new();
            for i in 0..nums.len() {
                let mut new_nums: Vec<usize> = nums[0..i].to_vec();
                new_nums.extend(&nums[i + 1..]);
                options.push(new_nums);
            }
            options
                .into_iter()
                .map(|v| is_safe(&v))
                .max()
                .expect("We know we have at least one.")
        })
        .sum::<usize>()
        .to_string()
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin()
        .lines()
        .collect::<std::io::Result<Vec<String>>>()?;
    println!("Part 1: {}", part1(lines.clone().into_iter()));
    println!("Part 2: {}", part2(lines.into_iter()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE.lines().map(ToOwned::to_owned)), "4");
    }
}
