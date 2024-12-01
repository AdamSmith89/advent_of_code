use crate::error::AdventError;

type ParsedInput = (Vec<usize>, Vec<usize>);

pub fn parse(input: &str) -> color_eyre::Result<ParsedInput> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let mut splits = line.split_whitespace();
        let v1 = splits.next().ok_or(AdventError::EndOfIterator)?.parse()?;
        list1.push(v1);

        let v2 = splits.next().ok_or(AdventError::EndOfIterator)?.parse()?;
        list2.push(v2);
    }

    Ok((list1, list2))
}

pub fn part1((list1, list2): &ParsedInput) -> color_eyre::Result<usize> {
    let mut list1 = list1.clone();
    list1.sort();

    let mut list2 = list2.clone();
    list2.sort();

    Ok(list1
        .iter()
        .zip(list2.iter())
        .map(|(v1, v2)| v1.abs_diff(*v2))
        .sum())
}

pub fn part2((list1, list2): &ParsedInput) -> color_eyre::Result<usize> {
    Ok(list1
        .iter()
        .map(|v1| {
            let occ = list2.iter().filter(|&v2| v1 == v2).count();

            v1 * occ
        })
        .sum())
}
