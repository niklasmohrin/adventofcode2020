use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Seat {
    Floor,
    Free,
    Occupied,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Visibility {
    Adjacent,
    Far,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeatingArea {
    seats: Vec<Vec<Seat>>,
}

impl SeatingArea {
    pub fn occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|&&s| s == Seat::Occupied).count())
            .sum()
    }

    pub fn stabilize(&mut self, visibility: Visibility) -> usize {
        let mut new_seats = self.seats.clone();
        let mut rounds = 0;
        loop {
            rounds += 1;
            self.apply_round_into(&mut new_seats, visibility);
            if new_seats == self.seats {
                break;
            }
            self.seats.clone_from(&new_seats);
        }
        rounds
    }

    fn apply_round_into(&self, new_seats: &mut Vec<Vec<Seat>>, visibility: Visibility) {
        let walk_away_limit = match visibility {
            Visibility::Adjacent => 4,
            Visibility::Far => 5,
        };
        for (i, row) in self.seats.iter().enumerate() {
            for (j, &seat) in row.iter().enumerate() {
                let adjacent = self.adjacent_to(i, j, visibility);
                new_seats[i][j] = if seat == Seat::Free && adjacent == 0 {
                    Seat::Occupied
                } else if seat == Seat::Occupied && adjacent >= walk_away_limit {
                    Seat::Free
                } else {
                    seat
                };
            }
        }
    }

    fn adjacent_to(&self, row: usize, col: usize, visibility: Visibility) -> usize {
        let mut c = 0;

        if visibility == Visibility::Adjacent {
            for dx in -1..=1 {
                if let Some(row) = self.seats.get((row as isize + dx) as usize) {
                    for dy in -1..=1 {
                        if !(dx == 0 && dy == 0)
                            && row.get((col as isize + dy) as usize) == Some(&Seat::Occupied)
                        {
                            c += 1;
                        }
                    }
                }
            }
        } else if visibility == Visibility::Far {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx != 0 || dy != 0 {
                        if self.sees_occupied_in_direction_from_position(row, col, dx, dy) {
                            c += 1;
                        }
                    }
                }
            }
        } else {
            unreachable!();
        }
        c
    }

    fn sees_occupied_in_direction_from_position(
        &self,
        row: usize,
        col: usize,
        dx: isize,
        dy: isize,
    ) -> bool {
        let mut x = col as isize;
        let mut y = row as isize;

        loop {
            x += dx;
            y += dy;

            if let Some(row) = self.seats.get(y as usize) {
                if let Some(seat) = row.get(x as usize) {
                    match seat {
                        Seat::Occupied => return true,
                        Seat::Free => return false,
                        Seat::Floor => continue,
                    }
                } else {
                    // Column out of bounds, no occupied seats coming anymore
                    return false;
                }
            } else {
                // Row out of bounds, no occupied seats coming anymore
                return false;
            }
        }
    }
}

impl FromStr for SeatingArea {
    type Err = <Seat as TryFrom<char>>::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seats = s
            .strip_suffix("\n")
            .unwrap_or(s)
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(Seat::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        Ok(Self { seats })
    }
}

impl TryFrom<char> for Seat {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Seat::Floor),
            'L' => Ok(Seat::Free),
            '#' => Ok(Seat::Occupied),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &'static str = include_str!("../small");

    #[test]
    fn p1_small() {
        let mut area = SMALL_EXAMPLE
            .parse::<SeatingArea>()
            .expect("Couldn't parse small example");
        area.stabilize(Visibility::Adjacent);
        assert_eq!(area.occupied(), 37);
    }

    #[test]
    fn p2_small() {
        let mut area = SMALL_EXAMPLE
            .parse::<SeatingArea>()
            .expect("Couldn't parse small example");
        area.stabilize(Visibility::Far);
        assert_eq!(area.occupied(), 26);
    }
}
