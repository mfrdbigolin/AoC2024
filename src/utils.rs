// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

/// Given a vector, creates a hash map that associates to each element its
/// corresponding count of occurrences in the vector.
pub fn create_count_map<T>(vec: &Vec<T>) -> std::collections::HashMap<T, u32>
where
    T: Eq,
    T: Copy,
    T: std::hash::Hash,
{
    let mut count_map: std::collections::HashMap<T, u32> = std::collections::HashMap::new();

    for &elem in vec {
        *count_map.entry(elem).or_default() += 1;
    }

    count_map
}
