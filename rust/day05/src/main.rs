fn get_middle<A>(slice: &[A]) -> &A {
    assert_eq!(slice.len() % 2, 1);
    &slice[slice.len() / 2]
}

fn parse_input(mut lines: impl Iterator<Item = String>) -> (Vec<[usize; 2]>, Vec<Vec<usize>>) {
    let mut line = lines.next().expect("We know there's at least one line.");
    let mut relations: Vec<[usize; 2]> = Vec::new();
    
    while !line.is_empty() {
        let mut line_split = line.split('|').map(|s| s.parse::<usize>().unwrap());
        relations.push([line_split.next().unwrap(), line_split.next().unwrap()]);

        line = lines
            .next()
            .expect("We know there's at least one empty line.");
    }
    let updates: Vec<Vec<usize>> = lines
        .map(|l| l.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();

    (relations, updates)
}

fn part1(lines: impl Iterator<Item = String>) -> String {
    let (relations, updates) = parse_input(lines);

    // We find all the correct updates...
    let correct_updates: Vec<Vec<usize>> = updates
        .into_iter()
        .filter(|l| {
            // ... by filtering based on whether a swapped order appears in the relations vec. If it
            // does, that implies that it violated an ordering rule.
            for i in 0..l.len() {
                for n in &l[i..] {
                    if relations.contains(&[*n, l[i]]) {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    // We now find the middle value, sum them, and return that as our answer.
    correct_updates
        .into_iter()
        .map(|update| *get_middle(update.as_slice()))
        .sum::<usize>()
        .to_string()
}

fn part2(lines: impl Iterator<Item = String>) -> String {
    let (relations, updates) = parse_input(lines);

    // We first find all incorrect updates by essentially doing the same as part 1 but swapping the
    // filter function output.
    let incorrect_updates: Vec<Vec<usize>> = updates
        .into_iter()
        .filter(|l| {
            for i in 0..l.len() {
                for n in &l[i..] {
                    if relations.contains(&[*n, l[i]]) {
                        return true;
                    }
                }
            }
            false
        })
        .collect();

    // Then we correct the mistakes. We have a stack of all the incorrect updates. When we find a
    // mistake, we'll swap the violating pair and re-add the update to the stack (because we may
    // have introduced a new mistake). Eventually, after many swaps, we will have corrected all
    // mistakes.
    let mut to_correct_stack: Vec<Vec<usize>> = incorrect_updates;
    let mut corrected_updates: Vec<Vec<usize>> = Vec::new();
    while let Some(mut update) = to_correct_stack.pop() {
        // We need this break_out variables for two reasons:
        // 1. to break out of two for loops,
        // 2. to tell if an update is now correct.
        // It will be set to true if we swapped.
        let mut break_out: bool = false;
        for i in 0..update.len() {
            for j in i..update.len() {
                if relations.contains(&[update[j], update[i]]) {
                    update.swap(i, j);
                    break_out = true;
                }
                if break_out {
                    break;
                }
            }
            if break_out {
                break;
            }
        }
        if break_out {
            to_correct_stack.push(update);
        } else {
            corrected_updates.push(update);
        }
    }

    // Finally, we get the middle value as before.
    corrected_updates
        .into_iter()
        .map(|update| *get_middle(update.as_slice()))
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

    const SAMPLE1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE1.lines().map(ToOwned::to_owned)), "143");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE1.lines().map(ToOwned::to_owned)), "123");
    }
}
