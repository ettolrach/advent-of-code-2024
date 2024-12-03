use std::collections::VecDeque;

// A bit of inspection of the input reveals two important things.
//
// 1. We don't have non-ASCII characters, or more specifically, characters which couldn't be
//    properly parsed using the str::chars() function.
// 2. The newlines are placed seemingly arbitrarily, so we can simply remove all newlines without
//    creating mistakes.
//
// We will take advantage of these :3

/// Calculate all the results of the `mul` instructions in a string.
fn products(mut cs: VecDeque<char>) -> Vec<usize> {
    let mut to_return: Vec<usize> = Vec::new();
    // "mul(0,1)" is an instance of the shortest acceptable instructions. So, we need there to
    // be at least 8 still left in the queue.
    while cs.len() > 7 {
        // Unfortunately, VecDeque doesn't implement slices, so we can't do &cs[0..4].
        if (cs[0], cs[1], cs[2], cs[3]) == ('m', 'u', 'l', '(') {
            let _ = cs.pop_front();
            let _ = cs.pop_front();
            let _ = cs.pop_front();
            let _ = cs.pop_front();
            let (mut lhs_chars, mut rhs_chars): (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());

            // If we find an inner "mul(" then we need to quite early and not try to parase the RHS.
            // We'll use this skip boolean to skip over the RHS when necessary.
            let mut skip: bool = false;

            // Get the LHS digits.
            while let Some(c) = cs.pop_front() {
                if c == ',' {
                    break;
                }
                // Else if we have an inner "mul(", we need to end the current parse attempt and
                // skip ahead.
                else if (c == 'm')
                    && (cs.front() == Some(&'u'))
                    && (cs.get(1) == Some(&'l'))
                    && (cs.get(2) == Some(&'('))
                {
                    // Put the 'm' back to parse again.
                    cs.push_front(c);
                    skip = true;
                    break;
                }
                lhs_chars.push(c);
            }
            if skip {
                continue;
            }
            // If we ran out, it's invalid.
            if cs.is_empty() {
                return to_return;
            }
            let mut finished: bool = false;
            while let Some(c) = cs.pop_front() {
                // If we have a closing bracket, we're done.
                if c == ')' {
                    finished = true;
                    break;
                }
                // Else if we have an inner "mul(", we need to end the current parse attempt and
                // skip ahead.
                else if (c == 'm')
                    && (cs.front() == Some(&'u'))
                    && (cs.get(1) == Some(&'l'))
                    && (cs.get(2) == Some(&'('))
                {
                    // Put the 'm' back to parse again.
                    cs.push_front(c);
                    finished = true;
                    break;
                }
                rhs_chars.push(c);
            }
            // If we didn't find a closing bracket but we did reach the end of the string,
            // return.
            if !finished {
                return to_return;
            }

            let maybe_lhs: Result<usize, _> = lhs_chars.into_iter().collect::<String>().parse();
            let maybe_rhs: Result<usize, _> = rhs_chars.into_iter().collect::<String>().parse();
            // If they aren't valid numbers, don't add them to the products.
            if maybe_lhs.is_err() || maybe_rhs.is_err() {
                continue;
            }
            to_return.push(maybe_lhs.unwrap() * maybe_rhs.unwrap());
        } else {
            let _ = cs.pop_front();
        }
    }
    to_return
}

fn part1(input: &str) -> String {
    products(input.chars().collect()).into_iter().sum::<usize>().to_string()
}

fn part2(input: &str) -> String {
    let mut dont_split = input.split("don't()");
    let mut dos: Vec<&str> = vec![
        dont_split.next().expect("We know there's at least one don't.")
    ];
    let mut donts: Vec<&str> = Vec::new();
    // For each dont_segment, we split at "do()", add the first of the split to the donts,
    // and the rest to the dos.
    for dont_segment in dont_split {
        let mut do_split = dont_segment.split("do()");
        // Add the first part to the donts.
        donts.push(do_split.next().expect("We know there's at least one part."));
        // If there are no more parts, then all good! Otherwise, we need to add them to the
        // dos.
        dos.extend(do_split);
    }
    products(dos.join("").chars().collect()).into_iter().sum::<usize>().to_string()
}

fn main() -> std::io::Result<()> {
    let input: String = std::io::read_to_string(std::io::stdin())?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE1), "161");
    }

    #[test]
    fn test_products() {
        let text: &str = "abcmul(1,2)xxxmul(5, 5)mul(345,984)mul(3,4)";
        let expected: Vec<usize> = vec![2, 339480, 12];
        assert_eq!(expected, products(text.chars().collect()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2), "48");
    }
}
