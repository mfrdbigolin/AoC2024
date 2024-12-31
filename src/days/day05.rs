// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Five, Print Queue.

// It is possible to simply use the sort_by method with the rules. However, since I had already
// implemented the current topological sort method, I decided to keep it.
fn solve(rule_graph: &AdjacencyList, updates: &Vec<Vec<u32>>) -> (u32, u32) {
    let mut sum_correct = 0;
    let mut sum_corrected = 0;

    for page in updates {
        let selected_page_nums: std::collections::HashSet<u32> =
            page.into_iter().cloned().collect();
        let sorted_page = topological_sort(rule_graph, &selected_page_nums);

        if *page == sorted_page {
            sum_correct += page[page.len() / 2];
        } else {
            sum_corrected += sorted_page[sorted_page.len() / 2];
        }
    }

    (sum_correct, sum_corrected)
}

fn parse_input(rules_and_updates_str: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules_str, updates_str) = rules_and_updates_str
        .split_once("\n\n")
        .expect("there should be two sections separated with a blank line");

    let rules = rules_str
        .lines()
        .map(|line| {
            let (src, dst) = line
                .split_once('|')
                .expect("a page ordering rule should be delimited with a '|'");

            (
                src.parse()
                    .expect("the first component of a rule has to be a number"),
                dst.parse()
                    .expect("the second component of a rule has to be a number"),
            )
        })
        .collect();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| {
                    num_str
                        .parse()
                        .expect("each update should be formed by the page numbers")
                })
                .collect()
        })
        .collect();

    (rules, updates)
}

type AdjacencyList = std::collections::HashMap<u32, std::collections::HashSet<u32>>;

fn build_rule_graph(rules: &Vec<(u32, u32)>) -> AdjacencyList {
    let mut rule_graph: AdjacencyList = std::collections::HashMap::new();

    for (src, dst) in rules {
        rule_graph.entry(*src).or_default().insert(*dst);
    }

    rule_graph
}

fn dfs(
    u: u32,
    graph: &AdjacencyList,
    active_vertices: &std::collections::HashSet<u32>,
    visited: &mut std::collections::HashSet<u32>,
    ts: &mut Vec<u32>,
) {
    visited.insert(u);

    for &v in graph
        .get(&u)
        .unwrap_or(&std::collections::HashSet::<u32>::new())
        .into_iter()
    {
        if !active_vertices.contains(&v) {
            continue;
        }

        if visited.contains(&v) {
            continue;
        }

        dfs(v, graph, active_vertices, visited, ts);
    }

    ts.push(u);
}

fn topological_sort(
    graph: &AdjacencyList,
    active_vertices: &std::collections::HashSet<u32>,
) -> Vec<u32> {
    let mut ts: Vec<u32> = Vec::new();

    let mut visited: std::collections::HashSet<u32> = std::collections::HashSet::new();

    for &s in graph.keys() {
        if !active_vertices.contains(&s) {
            continue;
        }

        if visited.contains(&s) {
            continue;
        }

        dfs(s, graph, active_vertices, &mut visited, &mut ts);
    }

    ts.reverse();
    ts
}

pub fn day05(input_data: &str) {
    let (rules, updates) = parse_input(input_data);

    let rule_graph = build_rule_graph(&rules);

    let (sol1, sol2) = solve(&rule_graph, &updates);

    println!("{sol1}");
    println!("{sol2}");
}
