fn part1(lines: impl Iterator<Item = String>) -> String {
    let lines_parsed: Vec<Vec<usize>> = lines
        // We each line at the whitespace. For each line, we now have an iterator of usize, where
        // each element is one of the numbers.
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("We now AoC has good inputs."))
                // Ideally we'd now call a function which Haskell calls transpose. But Rust doesn't have
                // that! So instead we'll have to evaluate it now.
                .collect()
        })
        .collect();

    // We know that there are only two numbers, so we will manually do two loops.
    let mut left: Vec<usize> = lines_parsed.iter().map(|v| v[0]).collect();
    let mut right: Vec<usize> = lines_parsed.iter().map(|v| v[1]).collect();
    left.sort();
    right.sort();

    // Now we can continue doing this in a one-liner.
    std::iter::zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
        .to_string()
}

fn part2(lines: impl Iterator<Item = String>) -> String {
    let lines_parsed: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("We now AoC has good inputs."))
                .collect()
        })
        .collect();

    let left: Vec<usize> = lines_parsed.iter().map(|v| v[0]).collect();
    let right: Vec<usize> = lines_parsed.iter().map(|v| v[1]).collect();

    left.into_iter()
        .map(|n| n * right.iter().filter(|m| **m == n).count())
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
