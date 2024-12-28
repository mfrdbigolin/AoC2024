// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Four, Ceres Search.

fn solve1(word_matrix: &Vec<Vec<char>>, word: &str) -> u32 {
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let num_rows = word_matrix.len();
    let num_cols = word_matrix[0].len();

    let first_letter = word.chars().next().expect("word should not be empty");

    let mut count_occs = 0;

    for (i, line) in word_matrix.into_iter().enumerate() {
        for (j, &letter) in line.into_iter().enumerate() {
            if letter != first_letter {
                continue;
            }

            for (dir_i, dir_j) in DIRECTIONS {
                if word.chars().enumerate().any(|(delta, expected_letter)| {
                    let new_i = (i as isize) + (delta as isize) * dir_i;
                    let new_j = (j as isize) + (delta as isize) * dir_j;

                    if new_i < 0 || new_i >= (num_rows as isize) {
                        return true;
                    }

                    if new_j < 0 || new_j >= (num_cols as isize) {
                        return true;
                    }

                    return word_matrix[new_i as usize][new_j as usize] != expected_letter;
                }) {
                    continue;
                }

                count_occs += 1;
            }
        }
    }

    count_occs
}

fn solve2(word_matrix: &Vec<Vec<char>>, word: &str) -> u32 {
    const DIAGONAL_DIRECTIONS: [(isize, isize); 2] = [(1, -1), (1, 1)];

    assert!(word.len() % 2 == 1);

    let num_rows = word_matrix.len();
    let num_cols = word_matrix[0].len();

    let half_len = word.len() / 2;

    let pivot = word
        .chars()
        .skip(half_len)
        .next()
        .expect("word should not be empty");

    let mut count_occs = 0;

    for (i, line) in word_matrix
        .into_iter()
        .enumerate()
        .take(num_rows - half_len)
        .skip(half_len)
    {
        'letter_loop: for (j, &letter) in line
            .into_iter()
            .enumerate()
            .take(num_cols - half_len)
            .skip(half_len)
        {
            if letter != pivot {
                continue;
            }

            for (dir_i, dir_j) in DIAGONAL_DIRECTIONS {
                let base_i = (i as isize) - (half_len as isize) * dir_i;
                let base_j = (j as isize) - (half_len as isize) * dir_j;

                let is_expected_letter = |delta: usize, expected_letter: char| {
                    let new_i = (base_i as isize) + (delta as isize) * dir_i;
                    let new_j = (base_j as isize) + (delta as isize) * dir_j;

                    return word_matrix[new_i as usize][new_j as usize] == expected_letter;
                };

                if word
                    .chars()
                    .enumerate()
                    .any(|(k, letter)| !is_expected_letter(k, letter))
                    && word
                        .chars()
                        .rev()
                        .enumerate()
                        .any(|(k, letter)| !is_expected_letter(k, letter))
                {
                    continue 'letter_loop;
                }
            }

            count_occs += 1;
        }
    }

    count_occs
}

fn parse_input(word_matrix_str: &str) -> Vec<Vec<char>> {
    word_matrix_str
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn day04(input_data: &str) {
    let word_matrix = parse_input(input_data);

    let sol1 = solve1(&word_matrix, "XMAS");
    let sol2 = solve2(&word_matrix, "MAS");

    println!("{sol1}");
    println!("{sol2}");
}
