use template::part1::solve1;


fn main() {
    let input = include_str!("../../input/part1.txt");
    let output = solve1::solve(input);
    println!("Part 1: {}", output);
}

