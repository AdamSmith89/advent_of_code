pub trait Solver {
    type ParsedInput;
    type Output;

    fn parse(input: &str) -> color_eyre::Result<Self::ParsedInput>;
    fn part1(&self, input: &Self::ParsedInput) -> color_eyre::Result<Self::Output>;
    fn part2(&self, input: &Self::ParsedInput) -> color_eyre::Result<Self::Output>;
}
