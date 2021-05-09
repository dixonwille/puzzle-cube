use crate::cubit::Cubit;
use crate::error::Error;
use nalgebra::Vector3;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Cube {
    sides: usize,
    // fix: when you can start evaluating generics, change this vec to use an array
    cubits: Vec<Cubit>,
}

impl Cube {
    pub fn with_number_sides(sides: usize) -> Result<Self, Error> {
        if sides < 2 {
            return Err(Error::InvalidNumberSides(sides));
        }
        let full = sides.pow(3);
        let size = full - (sides - 2).pow(3);
        let mut cube = Cube {
            sides,
            cubits: Vec::with_capacity(size),
        };
        let indexes: Vec<Vector3<isize>> = (0..full)
            .filter_map(|i| {
                let v = cube.index_to_coords(i);
                let (min_outer, max_outer) = if cube.even_sides() {
                    ((cube.sides - 1) as isize * -1, (cube.sides - 1) as isize)
                } else {
                    ((cube.sides / 2) as isize * -1, (cube.sides / 2) as isize)
                };
                if v[(0)] == min_outer
                    || v[(0)] == max_outer
                    || v[(1)] == min_outer
                    || v[(1)] == max_outer
                    || v[(2)] == min_outer
                    || v[(2)] == max_outer
                {
                    Some(v)
                } else {
                    None
                }
            })
            .collect(); // Have to collect them before iterating, becuase we reference cube immutably in the iterator
        for v in indexes {
            cube.cubits.push(Cubit::std_from_position(v))
        }
        Ok(cube)
    }

    pub fn new2x2x2() -> Self {
        Self::with_number_sides(2).expect("2 is a valid number of sides")
    }

    pub fn new3x3x3() -> Self {
        Self::with_number_sides(3).expect("3 is a valid number of sides")
    }

    fn index_to_coords(&self, idx: usize) -> Vector3<isize> {
        if self.even_sides() {
            Vector3::new(
                (2 * ((idx / self.sides) % self.sides)) as isize - (self.sides - 1) as isize,
                (2 * (idx % self.sides)) as isize - (self.sides - 1) as isize,
                (2 * ((idx / self.sides.pow(2)) % self.sides)) as isize - (self.sides - 1) as isize,
            )
        } else {
            Vector3::new(
                ((idx / self.sides) % self.sides) as isize - (self.sides / 2) as isize,
                (idx % self.sides) as isize - (self.sides / 2) as isize,
                ((idx / self.sides.pow(2)) % self.sides) as isize - (self.sides / 2) as isize,
            )
        }
    }

    #[inline]
    fn even_sides(&self) -> bool {
        self.sides % 2 == 0
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
}
