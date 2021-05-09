use nalgebra::{Matrix3x4, Vector3};

/// A Cubit a single piece of the whole puzzle. It has information about its
/// position and orientation inside of the whole cube.
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(crate) struct Cubit {
    // the Position in first column (x, y, z)
    // the Blue/Green Vector in the 2nd column (x-axis blue positive)
    // the Red/Orange Vector in the 3rd column (y-axis red positive)
    // the Yellow/White Vector in the 4th column (z-axis yellow positive)
    inner: Matrix3x4<isize>,
}

impl Cubit {
    fn new(
        pos: Vector3<isize>,
        xaxis: Vector3<isize>,
        yaxis: Vector3<isize>,
        zaxis: Vector3<isize>,
    ) -> Self {
        let inner = Matrix3x4::from_columns(&[pos, xaxis, yaxis, zaxis]);
        Cubit { inner }
    }

    /// Create a Cubit in the standard orientation at a given postion.
    /// Creating a Cube where all Cubits are created with this function will create
    /// a solved Cube.
    pub(crate) fn std_from_position(pos: Vector3<isize>) -> Self {
        let (ox, oy, oz) = new_std_orientation();
        Self::new(pos, ox, oy, oz)
    }
}

#[inline]
fn new_std_orientation() -> (Vector3<isize>, Vector3<isize>, Vector3<isize>) {
    (Vector3::x(), Vector3::y(), Vector3::z())
}
