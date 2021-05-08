use nalgebra::Vector3;

use crate::cubit::Cubit;
use crate::error::Error;

#[derive(Debug)]
struct Cube {
    sides: usize,
    // FIX: When you can start evaluating generics, change this Vec to use an array
    cubits: Vec<Cubit>,
}

impl Cube {
    fn with_number_sides(sides: usize) -> Result<Self, Error> {
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

    fn new2x2x2() -> Self {
        Self::with_number_sides(2).expect("2 is a valid number of sides")
    }

    fn new3x3x3() -> Self {
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
