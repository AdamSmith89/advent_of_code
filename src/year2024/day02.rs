use itertools::Itertools;


type ParsedInput = Vec<Vec<u32>>;

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    input
        .lines()
        .map(|line| {
            Ok(line
                .split(' ')
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()?)
        })
        .collect::<Result<ParsedInput, _>>()
}

// How many reports are safe
// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
pub fn part1(reports: &ParsedInput) -> color_eyre::Result<usize> {
    Ok(reports.iter().filter(is_report_safe).count())
}

// Now, the same rules apply as before, except if removing a single level from an unsafe report
// would make it safe, the report instead counts as safe.
pub fn part2(reports: &ParsedInput) -> color_eyre::Result<usize> {
    let mut safe_count = 0;

    for report in reports {
        if is_report_safe(&report) {
            safe_count += 1;
        } else {
            for idx in 0..report.len() {
                let x = [&report[..idx], &report[idx + 1..]].concat();
                if is_report_safe(&&x) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    Ok(safe_count)
}

fn is_report_safe(report: &&Vec<u32>) -> bool {
    (report.is_sorted() || report.iter().rev().is_sorted())
        && report.iter().tuple_windows().all(|(lhs, rhs)| {
            let diff = lhs.abs_diff(*rhs);
            (1..=3).contains(&diff)
        })
}
