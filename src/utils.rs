pub const LINE_FEED: u8 = 10;
pub const CARRIAGE_RETURN: u8 = 13;

pub fn parse_from_bytes(input: &[u8]) -> impl Iterator<Item = impl Iterator<Item = u8> + '_> {
    input
        .split(|int| *int == LINE_FEED)
        .filter(|line| !line.is_empty())
        .map(|line| line.iter().filter(|c| *c != &CARRIAGE_RETURN).copied())
}