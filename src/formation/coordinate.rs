#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl Coordinate {
    /// If the column has an odd number of slots, `x` must be odd.
    /// This means that the leftmost column for descent into darkness
    /// should be x = 1, and for world's wake it should be x = 0.
    ///
    /// See the tests for example formations
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x, y }
    }

    pub fn is_adjacent_to(&self, coord: &Coordinate) -> bool {
        self != coord &&
            (self.adjacent_in_same_column(coord) ||
            self.adjacent_in_different_column(coord))
    }

    fn adjacent_in_same_column(&self, coord: &Coordinate) -> bool {
        self.x == coord.x &&
            coord.y >= self.y.saturating_sub(1) &&
            coord.y <= self.y + 1
    }

    fn adjacent_in_different_column(&self, coord: &Coordinate) -> bool {
        self.in_adjacent_column_to(coord) &&
            if self.x % 2 != 0 {
                (coord.y == self.y || coord.y == self.y + 1)
            } else {
                (coord.y == self.y.saturating_sub(1) || coord.y == self.y)
            }
    }

    fn in_adjacent_column_to(&self, coord: &Coordinate) -> bool {
        self.x == coord.x.saturating_sub(1) ||
            self.x == coord.x + 1
    }
}

#[test]
fn test_worlds_wake_formation_has_correct_adjacency_numbers() {
    let formation = vec![
        Coordinate::new(0, 0),
        Coordinate::new(0, 1),
        Coordinate::new(0, 2),
        Coordinate::new(0, 3),
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
        Coordinate::new(1, 2),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 1),
    ];

    assert_adjacency_is_reflexive(&formation);

    assert_eq!(num_adjacent(&formation, 0, 0), 2);
    assert_eq!(num_adjacent(&formation, 0, 1), 4);
    assert_eq!(num_adjacent(&formation, 0, 2), 4);
    assert_eq!(num_adjacent(&formation, 0, 3), 2);
    assert_eq!(num_adjacent(&formation, 1, 0), 4);
    assert_eq!(num_adjacent(&formation, 1, 1), 6);
    assert_eq!(num_adjacent(&formation, 1, 2), 4);
    assert_eq!(num_adjacent(&formation, 2, 1), 4);
    assert_eq!(num_adjacent(&formation, 2, 2), 4);
    assert_eq!(num_adjacent(&formation, 3, 1), 2);
}

#[test]
fn test_descent_into_darkness_formation_has_correct_adjacency_numbers() {
    let formation = vec![
        Coordinate::new(1, 1),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 0),
        Coordinate::new(3, 1),
        Coordinate::new(3, 2),
        Coordinate::new(4, 1),
        Coordinate::new(4, 2),
        Coordinate::new(5, 1),
    ];

    assert_adjacency_is_reflexive(&formation);

    assert_eq!(num_adjacent(&formation, 1, 1), 2);
    assert_eq!(num_adjacent(&formation, 2, 1), 4);
    assert_eq!(num_adjacent(&formation, 2, 2), 4);
    assert_eq!(num_adjacent(&formation, 3, 0), 3);
    assert_eq!(num_adjacent(&formation, 3, 1), 6);
    assert_eq!(num_adjacent(&formation, 3, 2), 3);
    assert_eq!(num_adjacent(&formation, 4, 1), 4);
    assert_eq!(num_adjacent(&formation, 4, 2), 4);
    assert_eq!(num_adjacent(&formation, 5, 1), 2);
}

#[test]
fn test_ghostbeards_greed_formation_has_correct_adjacency_numbers() {
    let formation = vec![
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
        Coordinate::new(1, 2),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 0),
        Coordinate::new(3, 1),
        Coordinate::new(3, 2),
        Coordinate::new(4, 1),
        Coordinate::new(4, 2),
        Coordinate::new(5, 0),
        Coordinate::new(5, 1),
        Coordinate::new(5, 2),
    ];

    assert_adjacency_is_reflexive(&formation);

    assert_eq!(num_adjacent(&formation, 1, 0), 2);
    assert_eq!(num_adjacent(&formation, 1, 1), 4);
    assert_eq!(num_adjacent(&formation, 1, 2), 2);
    assert_eq!(num_adjacent(&formation, 2, 1), 5);
    assert_eq!(num_adjacent(&formation, 2, 2), 5);
    assert_eq!(num_adjacent(&formation, 3, 0), 3);
    assert_eq!(num_adjacent(&formation, 3, 1), 6);
    assert_eq!(num_adjacent(&formation, 3, 2), 3);
    assert_eq!(num_adjacent(&formation, 4, 1), 5);
    assert_eq!(num_adjacent(&formation, 4, 2), 5);
    assert_eq!(num_adjacent(&formation, 5, 0), 2);
    assert_eq!(num_adjacent(&formation, 5, 1), 4);
    assert_eq!(num_adjacent(&formation, 5, 2), 2);
}

#[test]
fn test_grimms_idle_tales_formation_has_correct_adjacency_numbers() {
    let formation = vec![
        Coordinate::new(0, 1),
        Coordinate::new(0, 2),
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
        Coordinate::new(1, 2),
        Coordinate::new(2, 1),
        Coordinate::new(2, 2),
        Coordinate::new(3, 1),
        Coordinate::new(4, 1),
        Coordinate::new(4, 2),
        Coordinate::new(5, 1),
    ];


    assert_adjacency_is_reflexive(&formation);

    assert_eq!(num_adjacent(&formation, 0, 1), 3);
    assert_eq!(num_adjacent(&formation, 0, 2), 3);
    assert_eq!(num_adjacent(&formation, 1, 0), 3);
    assert_eq!(num_adjacent(&formation, 1, 1), 6);
    assert_eq!(num_adjacent(&formation, 1, 2), 3);
    assert_eq!(num_adjacent(&formation, 2, 1), 4);
    assert_eq!(num_adjacent(&formation, 2, 2), 4);
    assert_eq!(num_adjacent(&formation, 3, 1), 4);
    assert_eq!(num_adjacent(&formation, 4, 1), 3);
    assert_eq!(num_adjacent(&formation, 4, 2), 3);
    assert_eq!(num_adjacent(&formation, 5, 1), 2);
}

#[cfg(test)]
fn num_adjacent(formation: &[Coordinate], x: u8, y: u8) -> usize {
    formation.iter()
        .filter(|c| c.is_adjacent_to(&Coordinate::new(x, y)))
        .count()
}

#[cfg(test)]
fn assert_adjacency_is_reflexive(formation: &[Coordinate]) {
    for coord in formation {
        for coord2 in formation {
            assert_eq!(
                coord.is_adjacent_to(coord2),
                coord2.is_adjacent_to(coord),
                "{:?} and {:?} are not reflexively adjacent",
                coord,
                coord2
            );
        }
    }
}
