// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Three, Mull It Over.

fn solve1(memory: &str) -> u32 {
    let mut product_sum = 0;

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [a, b]) in re.captures_iter(memory).map(|c| c.extract()) {
        let multiplicand: u32 = a.parse().unwrap();
        let multiplier: u32 = b.parse().unwrap();

        product_sum += multiplicand * multiplier;
    }

    product_sum
}

fn solve2(memory: &str) -> u32 {
    let mut product_sum = 0;

    let mut is_enabled = true;

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(()()\)|don\'t\(()()\)").unwrap();
    for (instr, [a, b]) in re.captures_iter(memory).map(|c| c.extract()) {
        if instr == "do()" || instr == "don't()" {
            is_enabled = instr == "do()";
            continue;
        }

        if !is_enabled {
            continue;
        }

        let multiplicand: u32 = a.parse().unwrap();
        let multiplier: u32 = b.parse().unwrap();

        product_sum += multiplicand * multiplier;
    }

    product_sum
}

pub fn day03(input_data: &str) {
    let sol1 = solve1(input_data);
    let sol2 = solve2(input_data);

    println!("{sol1}");
    println!("{sol2}");
}
