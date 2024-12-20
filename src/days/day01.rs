// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day One, Historian Hysteria.

use crate::utils;

fn solve1(loc_list1: &Vec<u32>, loc_list2: &Vec<u32>) -> u32 {
    assert!(loc_list1.is_sorted());
    assert!(loc_list2.is_sorted());

    std::iter::zip(loc_list1, loc_list2)
        .map(|(&loc_id1, &loc_id2)| loc_id1.abs_diff(loc_id2))
        .sum()
}

fn solve2(loc_list1: &Vec<u32>, loc_list2: &Vec<u32>) -> u32 {
    let loc_count1 = utils::create_count_map(&loc_list1);
    let loc_count2 = utils::create_count_map(&loc_list2);

    loc_count1
        .iter()
        .map(|(&loc_id, &id_count1)| {
            loc_id
                * id_count1
                * match loc_count2.get(&loc_id) {
                    Some(&id_count2) => id_count2,
                    None => 0,
                }
        })
        .sum()
}

fn parse_input(loc_lists_str: &str) -> (Vec<u32>, Vec<u32>) {
    let mut loc_list1: Vec<u32> = Vec::new();
    let mut loc_list2: Vec<u32> = Vec::new();

    for line in loc_lists_str.lines() {
        let mut split_iter = line.split_whitespace();

        let loc_id1: u32 = split_iter
            .next()
            .expect("there should not be an empty line")
            .parse()
            .expect("the first id should be an integer");

        let loc_id2: u32 = split_iter
            .next()
            .expect("there should be two location ids per line")
            .parse()
            .expect("the second id should be an integer");

        assert_eq!(
            None,
            split_iter.next(),
            "there should be only two location ids per line"
        );

        loc_list1.push(loc_id1);
        loc_list2.push(loc_id2);
    }

    (loc_list1, loc_list2)
}

pub fn day01(input_data: &str) {
    let (mut loc_list1, mut loc_list2) = parse_input(input_data);

    loc_list1.sort();
    loc_list2.sort();

    let sol1 = solve1(&loc_list1, &loc_list2);
    let sol2 = solve2(&loc_list1, &loc_list2);

    println!("{sol1}");
    println!("{sol2}");
}
