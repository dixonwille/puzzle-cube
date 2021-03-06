use nalgebra::{Matrix3, Matrix3x4, MatrixSlice3x1, Vector3};

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
        Self::new(pos, Vector3::x(), Vector3::y(), Vector3::z())
    }

    pub(crate) fn get_position(&self) -> MatrixSlice3x1<isize> {
        self.inner.column(0)
    }

    pub(crate) fn rotate(&mut self, rot: &Matrix3<isize>) {
        self.inner = rot * self.inner;
    }
}
