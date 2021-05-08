use nalgebra::Matrix3x4;

/// A Cubit a single piece of the whole puzzle. It has information about its
/// position and orientation inside of the whole cube.
pub struct Cubit {
    // the Position in first column (x, y, z)
    // the Blue/Green Vector in the 2nd column (x-axis blue positive)
    // the Red/Orange Vector in the 3rd column (y-axis red positive)
    // the Yellow/White Vector in the 4th column (z-axis yellow positive)
    inner: Matrix3x4<isize>,
}
