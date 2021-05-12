use std::convert::{TryFrom, TryInto};

use nalgebra::Matrix3;

use crate::error::Error;

pub(crate) enum LayerInner {
    Single(usize),
    Multiple(usize),
    WholeCube,
}

impl From<Layer> for LayerInner {
    fn from(l: Layer) -> Self {
        match l {
            Layer::Single(s) => LayerInner::Single(s),
            Layer::Multiple(m) => LayerInner::Multiple(m),
        }
    }
}

#[derive(Clone)]
pub(crate) enum AxisInner {
    X,
    NegX,
    Y,
    NegY,
    Z,
    NegZ,
}

impl From<Axis> for AxisInner {
    fn from(a: Axis) -> Self {
        match a {
            Axis::X => AxisInner::X,
            Axis::Y => AxisInner::Y,
            Axis::Z => AxisInner::Z,
        }
    }
}

/// Which layer(s) to affect when making the move
pub enum Layer {
    /// Affect a single layer (indexed at 0)
    Single(usize),
    /// Affect X number of layers
    /// If given 2 it will rotate the first 2 layers for the side specified
    Multiple(usize),
}

/// Axis to rotate cube around
pub enum Axis {
    X,
    Y,
    Z,
}

impl TryFrom<AxisInner> for Axis {
    type Error = Error;

    fn try_from(value: AxisInner) -> Result<Self, Self::Error> {
        match value {
            AxisInner::X => Ok(Axis::X),
            AxisInner::Y => Ok(Axis::Y),
            AxisInner::Z => Ok(Axis::Z),
            AxisInner::NegX | AxisInner::NegY | AxisInner::NegZ => Err(Error::AxisConvert),
        }
    }
}

/// What type of move to do.
#[derive(Clone)]
pub enum MoveType {
    /// Rotate clockwise
    Clockwise,
    /// Rotate counter-clockwise
    CounterClockwise,
    /// Rotate twice
    Twice,
}

impl MoveType {
    fn opposite(&self) -> Self {
        match self {
            MoveType::Clockwise => MoveType::CounterClockwise,
            MoveType::CounterClockwise => MoveType::Clockwise,
            MoveType::Twice => MoveType::Twice,
        }
    }
}

/// Describe how to move the cube.
pub struct Move {
    move_type: MoveType,
    pub(crate) axis: AxisInner,
    pub(crate) affected_range: LayerInner,
}

impl Move {
    /// Rotate the top side of the cube.
    pub fn rotate_top(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::Z,
            affected_range: layer.into(),
        }
    }

    /// Rotate the bottom side of the cube.
    pub fn rotate_bottom(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::NegZ,
            affected_range: layer.into(),
        }
    }

    /// Rotate the left side of the cube.
    pub fn rotate_left(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::NegY,
            affected_range: layer.into(),
        }
    }

    /// Rotate the right side of the cube.
    pub fn rotate_right(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::Y,
            affected_range: layer.into(),
        }
    }

    /// Rotate the front side of the cube.
    pub fn rotate_front(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::X,
            affected_range: layer.into(),
        }
    }

    /// Rotate the back side of the cube.
    pub fn rotate_back(layer: Layer, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: AxisInner::NegX,
            affected_range: layer.into(),
        }
    }

    /// Rotate the whole cube around an axis.
    pub fn rotate_cube(axis: Axis, move_type: MoveType) -> Self {
        Move {
            move_type,
            axis: axis.into(),
            affected_range: LayerInner::WholeCube,
        }
    }

    pub(crate) fn rotation_matrix(&self) -> &Matrix3<isize> {
        match self.normalize_axis_move_type() {
            (Axis::X, MoveType::Clockwise) => &ROT_MAT_X_CW,
            (Axis::X, MoveType::CounterClockwise) => &ROT_MAT_X_CCW,
            (Axis::X, MoveType::Twice) => &ROT_MAT_X_2,
            (Axis::Y, MoveType::Clockwise) => &ROT_MAT_Y_CW,
            (Axis::Y, MoveType::CounterClockwise) => &ROT_MAT_Y_CCW,
            (Axis::Y, MoveType::Twice) => &ROT_MAT_Y_2,
            (Axis::Z, MoveType::Clockwise) => &ROT_MAT_Z_CW,
            (Axis::Z, MoveType::CounterClockwise) => &ROT_MAT_Z_CCW,
            (Axis::Z, MoveType::Twice) => &ROT_MAT_Z_2,
        }
    }

    fn normalize_axis_move_type(&self) -> (Axis, MoveType) {
        match self.axis {
            AxisInner::NegX => (Axis::X, self.move_type.opposite()),
            AxisInner::NegY => (Axis::Y, self.move_type.opposite()),
            AxisInner::NegZ => (Axis::Z, self.move_type.opposite()),
            AxisInner::X | AxisInner::Y | AxisInner::Z => (
                self.axis
                    .clone()
                    .try_into()
                    .expect("dealing with values only supported by axis"),
                self.move_type.clone(),
            ),
        }
    }
}

static ROT_MAT_Z_CW: Matrix3<isize> = Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1);
static ROT_MAT_Z_CCW: Matrix3<isize> = Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1);
static ROT_MAT_Z_2: Matrix3<isize> = Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1);

static ROT_MAT_Y_CW: Matrix3<isize> = Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0);
static ROT_MAT_Y_CCW: Matrix3<isize> = Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0);
static ROT_MAT_Y_2: Matrix3<isize> = Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1);

static ROT_MAT_X_CW: Matrix3<isize> = Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0);
static ROT_MAT_X_CCW: Matrix3<isize> = Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0);
static ROT_MAT_X_2: Matrix3<isize> = Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotation_matrix() {
        let top_cw = Move::rotate_top(Layer::Single(0), MoveType::Clockwise);
        let top_ccw = Move::rotate_top(Layer::Single(0), MoveType::CounterClockwise);
        let top_2 = Move::rotate_top(Layer::Single(0), MoveType::Twice);

        let btm_cw = Move::rotate_bottom(Layer::Single(0), MoveType::Clockwise);
        let btm_ccw = Move::rotate_bottom(Layer::Single(0), MoveType::CounterClockwise);
        let btm_2 = Move::rotate_bottom(Layer::Single(0), MoveType::Twice);

        let left_cw = Move::rotate_left(Layer::Single(0), MoveType::Clockwise);
        let left_ccw = Move::rotate_left(Layer::Single(0), MoveType::CounterClockwise);
        let left_2 = Move::rotate_left(Layer::Single(0), MoveType::Twice);

        let right_cw = Move::rotate_right(Layer::Single(0), MoveType::Clockwise);
        let right_ccw = Move::rotate_right(Layer::Single(0), MoveType::CounterClockwise);
        let right_2 = Move::rotate_right(Layer::Single(0), MoveType::Twice);

        let front_cw = Move::rotate_front(Layer::Single(0), MoveType::Clockwise);
        let front_ccw = Move::rotate_front(Layer::Single(0), MoveType::CounterClockwise);
        let front_2 = Move::rotate_front(Layer::Single(0), MoveType::Twice);

        let back_cw = Move::rotate_back(Layer::Single(0), MoveType::Clockwise);
        let back_ccw = Move::rotate_back(Layer::Single(0), MoveType::CounterClockwise);
        let back_2 = Move::rotate_back(Layer::Single(0), MoveType::Twice);

        let cube_x_cw = Move::rotate_cube(Axis::X, MoveType::Clockwise);
        let cube_x_ccw = Move::rotate_cube(Axis::X, MoveType::CounterClockwise);
        let cube_x_2 = Move::rotate_cube(Axis::X, MoveType::Twice);

        let cube_y_cw = Move::rotate_cube(Axis::Y, MoveType::Clockwise);
        let cube_y_ccw = Move::rotate_cube(Axis::Y, MoveType::CounterClockwise);
        let cube_y_2 = Move::rotate_cube(Axis::Y, MoveType::Twice);

        let cube_z_cw = Move::rotate_cube(Axis::Z, MoveType::Clockwise);
        let cube_z_ccw = Move::rotate_cube(Axis::Z, MoveType::CounterClockwise);
        let cube_z_2 = Move::rotate_cube(Axis::Z, MoveType::Twice);

        // ROT_MAT_Z_CW
        assert_eq!(&ROT_MAT_Z_CW, top_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_CW, btm_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_CW, cube_z_cw.rotation_matrix());

        // ROT_MAT_Z_CCW
        assert_eq!(&ROT_MAT_Z_CCW, top_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_CCW, btm_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_CCW, cube_z_ccw.rotation_matrix());

        // ROT_MAT_Z_CCW
        assert_eq!(&ROT_MAT_Z_2, top_2.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_2, btm_2.rotation_matrix());
        assert_eq!(&ROT_MAT_Z_2, cube_z_2.rotation_matrix());

        // ROT_MAT_Y_CW
        assert_eq!(&ROT_MAT_Y_CW, right_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_CW, left_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_CW, cube_y_cw.rotation_matrix());

        // ROT_MAT_Y_CCW
        assert_eq!(&ROT_MAT_Y_CCW, right_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_CCW, left_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_CCW, cube_y_ccw.rotation_matrix());

        // ROT_MAT_Y_CCW
        assert_eq!(&ROT_MAT_Y_2, right_2.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_2, left_2.rotation_matrix());
        assert_eq!(&ROT_MAT_Y_2, cube_y_2.rotation_matrix());

        // ROT_MAT_X_CW
        assert_eq!(&ROT_MAT_X_CW, front_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_X_CW, back_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_X_CW, cube_x_cw.rotation_matrix());

        // ROT_MAT_X_CCW
        assert_eq!(&ROT_MAT_X_CCW, front_ccw.rotation_matrix());
        assert_eq!(&ROT_MAT_X_CCW, back_cw.rotation_matrix());
        assert_eq!(&ROT_MAT_X_CCW, cube_x_ccw.rotation_matrix());

        // ROT_MAT_X_CCW
        assert_eq!(&ROT_MAT_X_2, front_2.rotation_matrix());
        assert_eq!(&ROT_MAT_X_2, back_2.rotation_matrix());
        assert_eq!(&ROT_MAT_X_2, cube_x_2.rotation_matrix());
    }
}
