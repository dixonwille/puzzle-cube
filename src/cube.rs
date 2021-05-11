use crate::{
    cubit::Cubit,
    error::Error,
    movement::{LayerInner, Move},
};
use nalgebra::Vector3;
use std::ops::RangeInclusive;

/// Represents a full Puzzle Cube.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Cube {
    sides: usize,
    cubits: Vec<Cubit>,
}

impl Cube {
    /// Create a Puzzle Cube where sides is the numer of cubits on an edge.
    ///
    /// So `sides=10` would create a 10x10x10 cube.
    pub fn with_number_sides(sides: usize) -> Result<Self, Error> {
        if sides < 2 {
            return Err(Error::InvalidNumberSides(sides));
        }
        let full = sides.pow(3);
        let size = full - (sides - 2).pow(3);
        let mut cube = Cube {
            sides,
            cubits: Vec::with_capacity(0),
        };
        let cubits = (0..full)
            .filter_map(|i| {
                if ((i / cube.sides) % cube.sides) % (cube.sides - 1) == 0
                    || (i % cube.sides) % (cube.sides - 1) == 0
                    || (i / cube.sides.pow(2)) % (cube.sides - 1) == 0
                {
                    Some(Cubit::std_from_position(cube.index_to_coords(i)))
                } else {
                    None
                }
            })
            .fold(Vec::with_capacity(size), |mut v, c| {
                v.push(c);
                v
            });
        cube.cubits = cubits;
        Ok(cube)
    }

    /// Create a 2x2x2 Cube.
    pub fn new2x2x2() -> Self {
        Self::with_number_sides(2).expect("2 is a valid number of sides")
    }

    /// Create a 3x3x3 Cube.
    pub fn new3x3x3() -> Self {
        Self::with_number_sides(3).expect("3 is a valid number of sides")
    }

    /// Rotate the cube or sides given the move passed in.
    pub fn rotate(&mut self, mv: &Move) {
        let rot = mv.rotation_matrix();
        let mut x_range = self.ranges();
        let mut y_range = self.ranges();
        let mut z_range = self.ranges();
        // TODO: Figure out how to restrict ranges depending on layer and axis
        // Can be done in a bunch of conditionals but I wonder if there is a
        // better approach
        match &mv.affected_range {
            LayerInner::Single(l) => {}
            LayerInner::Multiple(r) => {}
            LayerInner::WholeCube => {
                // Nothing to do here as the whole cube is the default
            }
        }
        for c in self.cubits.iter_mut() {
            let pos = c.get_position();
            if x_range.contains(&pos[(0)])
                && y_range.contains(&pos[(1)])
                && z_range.contains(&pos[(2)])
            {
                c.rotate(rot);
            }
        }
    }

    fn index_to_coords(&self, idx: usize) -> Vector3<isize> {
        let offset = self.offset() as isize;
        let step = self.step() as isize;
        let x = ((idx / self.sides) % self.sides) as isize * step - offset;
        let y = (idx % self.sides) as isize * step - offset;
        let z = ((idx / self.sides.pow(2)) % self.sides) as isize * step - offset;
        Vector3::new(x, y, z)
    }

    fn even_sides(&self) -> bool {
        self.sides % 2 == 0
    }

    fn offset(&self) -> usize {
        if self.even_sides() {
            self.sides - 1
        } else {
            self.sides / 2
        }
    }

    fn step(&self) -> usize {
        if self.even_sides() {
            2
        } else {
            1
        }
    }

    fn ranges(&self) -> RangeInclusive<isize> {
        let offset = self.offset() as isize;
        RangeInclusive::new(offset * -1, offset)
    }
}
#[cfg(test)]
mod test {
    use super::Cube;
    use crate::cubit::Cubit;
    use crate::error::Error;
    use nalgebra::Vector3;

    #[test]
    fn test_invalid_side() {
        let maybe_cube = Cube::with_number_sides(1);
        match maybe_cube {
            Ok(_) => panic!("expected to get an error but didn't"),
            Err(e) => assert_eq!(e, Error::InvalidNumberSides(1)),
        }
    }

    #[test]
    fn test_2x2x2() {
        let cube = Cube::new2x2x2();
        let mut cubits = Vec::new();
        for z in -1..=1 {
            if z % 2 == 0 {
                continue;
            }
            for x in -1..=1 {
                if x % 2 == 0 {
                    continue;
                }
                for y in -1..=1 {
                    if y % 2 == 0 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 2, cubits })
    }

    #[test]
    fn test_3x3x3() {
        let cube = Cube::new3x3x3();
        let mut cubits = Vec::new();
        for z in -1..=1 {
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 3, cubits })
    }

    #[test]
    fn test_4x4x4() {
        let cube = Cube::with_number_sides(4).unwrap();
        let mut cubits = Vec::new();
        for z in -3..=3 {
            if z % 2 == 0 {
                continue;
            }
            for x in -3..=3 {
                if x % 2 == 0 {
                    continue;
                }
                for y in -3..=3 {
                    if y % 2 == 0 {
                        continue;
                    }
                    if x > -3 && x < 3 && y > -3 && y < 3 && z > -3 && z < 3 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 4, cubits })
    }

    #[test]
    fn test_5x5x5() {
        let cube = Cube::with_number_sides(5).unwrap();
        let mut cubits = Vec::new();
        for z in -2..=2 {
            for x in -2..=2 {
                for y in -2..=2 {
                    if x > -2 && x < 2 && y > -2 && y < 2 && z > -2 && z < 2 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 5, cubits })
    }

    #[test]
    fn test_99x99x99() {
        let cube = Cube::with_number_sides(99).unwrap();
        let mut cubits = Vec::new();
        for z in -49..=49 {
            for x in -49..=49 {
                for y in -49..=49 {
                    if x > -49 && x < 49 && y > -49 && y < 49 && z > -49 && z < 49 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 99, cubits })
    }

    #[test]
    fn test_100x100x100() {
        let cube = Cube::with_number_sides(100).unwrap();
        let mut cubits = Vec::new();
        for z in -99..=99 {
            if z % 2 == 0 {
                continue;
            }
            for x in -99..=99 {
                if x % 2 == 0 {
                    continue;
                }
                for y in -99..=99 {
                    if y % 2 == 0 {
                        continue;
                    }
                    if x > -99 && x < 99 && y > -99 && y < 99 && z > -99 && z < 99 {
                        continue;
                    }
                    cubits.push(Cubit::std_from_position(Vector3::new(x, y, z)))
                }
            }
        }
        assert_eq!(cube, Cube { sides: 100, cubits })
    }
}
