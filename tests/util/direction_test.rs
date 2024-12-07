use advent_of_code::util::direction::*;

#[test]
fn direction_is_opposite_of() {
    assert!(Direction::North.is_opposite_of(&Direction::South));
    assert!(Direction::East.is_opposite_of(&Direction::West));
    assert!(Direction::South.is_opposite_of(&Direction::North));
    assert!(Direction::West.is_opposite_of(&Direction::East));
}

#[test]
fn direction_get_opposite() {
    assert_eq!(Direction::South, Direction::North.get_opposite());
    assert_eq!(Direction::West, Direction::East.get_opposite());
    assert_eq!(Direction::North, Direction::South.get_opposite());
    assert_eq!(Direction::East, Direction::West.get_opposite());
}

#[test]
fn direction_rotate_90_cwise() {
    assert_eq!(Direction::East, Direction::North.rotate_90_cwise());
    assert_eq!(Direction::South, Direction::East.rotate_90_cwise());
    assert_eq!(Direction::West, Direction::South.rotate_90_cwise());
    assert_eq!(Direction::North, Direction::West.rotate_90_cwise());
}

#[test]
fn direction_rotate_90_c_cwise() {
    assert_eq!(Direction::West, Direction::North.rotate_90_c_cwise());
    assert_eq!(Direction::North, Direction::East.rotate_90_c_cwise());
    assert_eq!(Direction::East, Direction::South.rotate_90_c_cwise());
    assert_eq!(Direction::South, Direction::West.rotate_90_c_cwise());
}
