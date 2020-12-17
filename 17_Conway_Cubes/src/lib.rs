use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

#[cfg(feature = "three_dimensional")]
pub type Coordinate = (isize, isize, isize);
#[cfg(not(feature = "three_dimensional"))]
pub type Coordinate = (isize, isize, isize, isize);

pub const CELL_ACTIVE: char = '#';

#[derive(Clone)]
pub struct ConwayCubeWorld {
    world: HashSet<Coordinate>,
}

impl ConwayCubeWorld {
    pub fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
        let lines = BufReader::new(File::open(filename)?).lines();

        let mut world = HashSet::new();
        #[cfg(not(feature = "three_dimensional"))]
        let w = 0isize;
        let z = 0isize;

        for (y, line) in lines.enumerate() {
            for (x, val) in line?.chars().enumerate() {
                if val == CELL_ACTIVE {
                    #[cfg(feature = "three_dimensional")]
                    let cell = (x as isize, y as isize, z);
                    #[cfg(not(feature = "three_dimensional"))]
                    let cell = (x as isize, y as isize, z, z);
                    world.insert(cell);
                }
            }
        }

        Ok(Self { world })
    }

    pub fn cycle(&mut self) {
        let mut next_world_state = HashSet::new();

        for active_cell in self.world.iter().copied() {
            // Check every neighbor of an active cell, others won't be important
            next_world_state.extend(
                Self::neighbors_of(active_cell)
                    .iter()
                    .chain(iter::once(&active_cell))
                    .filter(|&&cell| self.should_be_active(cell)),
            )
        }

        self.world = next_world_state;
    }

    #[cfg(feature = "three_dimensional")]
    fn neighbors_of(cell: Coordinate) -> [Coordinate; 26] {
        let (x, y, z) = cell;
        [
            (-1 + x, -1 + y, -1 + z),
            (-1 + x, -1 + y, 0 + z),
            (-1 + x, -1 + y, 1 + z),
            (-1 + x, 0 + y, -1 + z),
            (-1 + x, 0 + y, 0 + z),
            (-1 + x, 0 + y, 1 + z),
            (-1 + x, 1 + y, -1 + z),
            (-1 + x, 1 + y, 0 + z),
            (-1 + x, 1 + y, 1 + z),
            (0 + x, -1 + y, -1 + z),
            (0 + x, -1 + y, 0 + z),
            (0 + x, -1 + y, 1 + z),
            (0 + x, 0 + y, -1 + z),
            (0 + x, 0 + y, 1 + z),
            (0 + x, 1 + y, -1 + z),
            (0 + x, 1 + y, 0 + z),
            (0 + x, 1 + y, 1 + z),
            (1 + x, -1 + y, -1 + z),
            (1 + x, -1 + y, 0 + z),
            (1 + x, -1 + y, 1 + z),
            (1 + x, 0 + y, -1 + z),
            (1 + x, 0 + y, 0 + z),
            (1 + x, 0 + y, 1 + z),
            (1 + x, 1 + y, -1 + z),
            (1 + x, 1 + y, 0 + z),
            (1 + x, 1 + y, 1 + z),
        ]
    }

    #[cfg(not(feature = "three_dimensional"))]
    fn neighbors_of(cell: Coordinate) -> [Coordinate; 80] {
        let (x, y, z, w) = cell;
        [
            (-1 + x, -1 + y, -1 + z, -1 + w),
            (-1 + x, -1 + y, -1 + z, 0 + w),
            (-1 + x, -1 + y, -1 + z, 1 + w),
            (-1 + x, -1 + y, 0 + z, -1 + w),
            (-1 + x, -1 + y, 0 + z, 0 + w),
            (-1 + x, -1 + y, 0 + z, 1 + w),
            (-1 + x, -1 + y, 1 + z, -1 + w),
            (-1 + x, -1 + y, 1 + z, 0 + w),
            (-1 + x, -1 + y, 1 + z, 1 + w),
            (-1 + x, 0 + y, -1 + z, -1 + w),
            (-1 + x, 0 + y, -1 + z, 0 + w),
            (-1 + x, 0 + y, -1 + z, 1 + w),
            (-1 + x, 0 + y, 0 + z, -1 + w),
            (-1 + x, 0 + y, 0 + z, 0 + w),
            (-1 + x, 0 + y, 0 + z, 1 + w),
            (-1 + x, 0 + y, 1 + z, -1 + w),
            (-1 + x, 0 + y, 1 + z, 0 + w),
            (-1 + x, 0 + y, 1 + z, 1 + w),
            (-1 + x, 1 + y, -1 + z, -1 + w),
            (-1 + x, 1 + y, -1 + z, 0 + w),
            (-1 + x, 1 + y, -1 + z, 1 + w),
            (-1 + x, 1 + y, 0 + z, -1 + w),
            (-1 + x, 1 + y, 0 + z, 0 + w),
            (-1 + x, 1 + y, 0 + z, 1 + w),
            (-1 + x, 1 + y, 1 + z, -1 + w),
            (-1 + x, 1 + y, 1 + z, 0 + w),
            (-1 + x, 1 + y, 1 + z, 1 + w),
            (0 + x, -1 + y, -1 + z, -1 + w),
            (0 + x, -1 + y, -1 + z, 0 + w),
            (0 + x, -1 + y, -1 + z, 1 + w),
            (0 + x, -1 + y, 0 + z, -1 + w),
            (0 + x, -1 + y, 0 + z, 0 + w),
            (0 + x, -1 + y, 0 + z, 1 + w),
            (0 + x, -1 + y, 1 + z, -1 + w),
            (0 + x, -1 + y, 1 + z, 0 + w),
            (0 + x, -1 + y, 1 + z, 1 + w),
            (0 + x, 0 + y, -1 + z, -1 + w),
            (0 + x, 0 + y, -1 + z, 0 + w),
            (0 + x, 0 + y, -1 + z, 1 + w),
            (0 + x, 0 + y, 0 + z, -1 + w),
            (0 + x, 0 + y, 0 + z, 1 + w),
            (0 + x, 0 + y, 1 + z, -1 + w),
            (0 + x, 0 + y, 1 + z, 0 + w),
            (0 + x, 0 + y, 1 + z, 1 + w),
            (0 + x, 1 + y, -1 + z, -1 + w),
            (0 + x, 1 + y, -1 + z, 0 + w),
            (0 + x, 1 + y, -1 + z, 1 + w),
            (0 + x, 1 + y, 0 + z, -1 + w),
            (0 + x, 1 + y, 0 + z, 0 + w),
            (0 + x, 1 + y, 0 + z, 1 + w),
            (0 + x, 1 + y, 1 + z, -1 + w),
            (0 + x, 1 + y, 1 + z, 0 + w),
            (0 + x, 1 + y, 1 + z, 1 + w),
            (1 + x, -1 + y, -1 + z, -1 + w),
            (1 + x, -1 + y, -1 + z, 0 + w),
            (1 + x, -1 + y, -1 + z, 1 + w),
            (1 + x, -1 + y, 0 + z, -1 + w),
            (1 + x, -1 + y, 0 + z, 0 + w),
            (1 + x, -1 + y, 0 + z, 1 + w),
            (1 + x, -1 + y, 1 + z, -1 + w),
            (1 + x, -1 + y, 1 + z, 0 + w),
            (1 + x, -1 + y, 1 + z, 1 + w),
            (1 + x, 0 + y, -1 + z, -1 + w),
            (1 + x, 0 + y, -1 + z, 0 + w),
            (1 + x, 0 + y, -1 + z, 1 + w),
            (1 + x, 0 + y, 0 + z, -1 + w),
            (1 + x, 0 + y, 0 + z, 0 + w),
            (1 + x, 0 + y, 0 + z, 1 + w),
            (1 + x, 0 + y, 1 + z, -1 + w),
            (1 + x, 0 + y, 1 + z, 0 + w),
            (1 + x, 0 + y, 1 + z, 1 + w),
            (1 + x, 1 + y, -1 + z, -1 + w),
            (1 + x, 1 + y, -1 + z, 0 + w),
            (1 + x, 1 + y, -1 + z, 1 + w),
            (1 + x, 1 + y, 0 + z, -1 + w),
            (1 + x, 1 + y, 0 + z, 0 + w),
            (1 + x, 1 + y, 0 + z, 1 + w),
            (1 + x, 1 + y, 1 + z, -1 + w),
            (1 + x, 1 + y, 1 + z, 0 + w),
            (1 + x, 1 + y, 1 + z, 1 + w),
        ]
    }

    fn should_be_active(&self, cell: Coordinate) -> bool {
        let count = Self::neighbors_of(cell)
            .iter()
            .filter(|neighbor| self.world.contains(&neighbor))
            .count();
        count == 3 || count == 2 && self.world.contains(&cell)
    }

    pub fn span(&self) -> (Coordinate, Coordinate) {
        let mut min_coords = Coordinate::default();
        let mut max_coords = Coordinate::default();

        for &t in self.world.iter() {
            min_coords.0 = min(min_coords.0, t.0);
            min_coords.1 = min(min_coords.1, t.1);
            min_coords.2 = min(min_coords.2, t.2);
            if cfg!(not(feature = "three_dimensional")) {
                min_coords.3 = min(min_coords.3, t.3);
            }

            max_coords.0 = max(max_coords.0, t.0);
            max_coords.1 = max(max_coords.1, t.1);
            max_coords.2 = max(max_coords.2, t.2);
            if cfg!(not(feature = "three_dimensional")) {
                max_coords.3 = max(max_coords.3, t.3);
            }
        }

        (min_coords, max_coords)
    }

    pub fn active_cubes(&self) -> usize {
        self.world.len()
    }
}

#[cfg(feature = "three_dimensional")]
impl fmt::Debug for ConwayCubeWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min_coord, max_coord) = self.span();

        for z in min_coord.2..=max_coord.2 {
            f.write_fmt(format_args!("z={}\n", z))?;
            for y in min_coord.1..=max_coord.1 {
                for x in min_coord.0..=max_coord.0 {
                    if self.world.contains(&(x, y, z)) {
                        f.write_str("#")?;
                    } else {
                        f.write_str(".")?;
                    }
                }
                f.write_str("\n")?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[cfg(not(feature = "three_dimensional"))]
impl fmt::Debug for ConwayCubeWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Spanning: {:?}\n", self.span()))
    }
}
