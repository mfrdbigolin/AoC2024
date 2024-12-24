// Copyright (C) 2024 Matheus Fernandes Bigolin <mfrdrbigolin@disroot.org>
// SPDX-License-Identifier: MIT

// Day Two, Red-Nosed Reports.

fn is_safe_and_incr(a: u32, b: u32) -> bool {
    let diff = (b as i64) - (a as i64);

    1 <= diff && diff <= 3
}

/// A breadth-first search on the implicit graph given by the reports levels
/// considering the tolerable number of deletions to determine whether the
/// report is safe.
fn bfs(report: &Vec<u32>, tolerance: u32) -> bool {
    let num_levels = report.len() as u32;

    // We consider two additional vertices that represent the start and end of the implicit graph.
    let source = 0u32;
    let sink = num_levels + 1;

    // We represent each vertex as a pair (i, k), where i is either a report level, the source or
    // the sink, and the weight k represents the number of intermediary edges. The weight is given
    // by the number of edges and each deletion is penalized with an additional edge.

    let mut visited: std::collections::HashSet<(u32, u32)> = std::collections::HashSet::new();
    visited.insert((source, 0));

    // Even given that the implicit graph is weighted, we can use BFS to
    // calculate distance by creating artificial vertices and edges to simulate
    // weight. This is only feasible because the edge weights scale with the
    // tolerance level (which is small for the problem given).
    let mut dist: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::new();
    dist.insert((source, 0), 0);

    let mut queue: std::collections::VecDeque<(u32, u32)> = std::collections::VecDeque::new();
    queue.push_front((source, 0));
    // Stop once the queue is empty or once the sink is reached.
    while !queue.is_empty() && !visited.contains(&(sink, 0)) {
        let (i, k) = queue.pop_front().unwrap();

        // The current vertex is an intermediary one.
        if k > 0 {
            if visited.contains(&(i, k - 1)) {
                continue;
            }
            visited.insert((i, k - 1));

            dist.insert((i, k - 1), dist[&(i, k)] + 1);

            queue.push_back((i, k - 1));

            continue;
        }

        // Consider every possible number of consecutive deletions.
        for num_deletions in 0..=std::cmp::min(tolerance, num_levels - i) {
            let j = i + num_deletions + 1;

            // An edge is only present in the implicit graph if we can safely
            // move between the levels given the current number of deletions.
            if i != source
                && j != sink
                && !is_safe_and_incr(report[(i - 1) as usize], report[(j - 1) as usize])
            {
                continue;
            }

            if visited.contains(&(j, 2 * num_deletions)) {
                continue;
            }
            visited.insert((j, 2 * num_deletions));

            dist.insert((j, 2 * num_deletions), dist[&(i, k)] + 1);

            queue.push_back((j, 2 * num_deletions));
        }
    }

    // The sink is unreachable from the source.
    if !visited.contains(&(sink, 0)) {
        return false;
    }

    // Considering that there are n + 2 vertices (all the levels plus the source and the sink),
    // there must be at least n + 1 edges between the sink and the source given our definition.
    // Considering how the deletions are penalized, for the report to be safe we can only tolerate
    // the number of deletions given as input.
    dist[&(sink, 0)] <= num_levels + 1 + tolerance
}

fn solve_report(report: &Vec<u32>, tolerance: u32) -> bool {
    // When all the elements except one are deleted, the report is trivially safe.
    if tolerance as usize >= report.len() - 1 {
        return true;
    }

    let mut report_rev = report.clone();
    report_rev.reverse();

    // We require that the elements in the report are strictly monotone, therefore
    // we can consider the report and the reversed report and require that one of
    // them has to be monotonically increasing. This idea is due to u/4HbQ from Reddit.
    bfs(report, tolerance) || bfs(&report_rev, tolerance)
}

fn solve(reports: &Vec<Vec<u32>>, tolerance: u32) -> u32 {
    reports
        .iter()
        .filter(|&report| solve_report(report, tolerance))
        .count() as u32
}

fn parse_input(reports_str: &str) -> Vec<Vec<u32>> {
    reports_str
        .lines()
        .map(|report_str| {
            report_str
                .split_whitespace()
                .map(|level_str| {
                    level_str
                        .parse()
                        .expect("each level in the report should be a number")
                })
                .collect()
        })
        .collect()
}

pub fn day02(input_data: &str) {
    let reports = parse_input(input_data);

    let sol1 = solve(&reports, 0);
    let sol2 = solve(&reports, 1);

    println!("{sol1}");
    println!("{sol2}");
}
