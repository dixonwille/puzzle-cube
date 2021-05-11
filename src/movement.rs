use std::{
    convert::{TryFrom, TryInto},
    ops::Range,
};

use nalgebra::Matrix3;

use crate::error::Error;

pub(crate) enum LayerInner {
    Single(usize),
    Multiple(Range<usize>),
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

pub enum Layer {
    Single(usize),
    Multiple(Range<usize>),
}

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

#[derive(Clone)]
pub enum MoveType {
    Clockwise,
    CounterClockwise,
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

pub struct Move {
    move_type: MoveType,
    pub(crate) axis: AxisInner,
    pub(crate) affected_range: LayerInner,
}

impl Move {
    /// Rotate the top side of the cube.
    pub fn rotate_top(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::Z,
            affected_range: layer.into(),
        })
    }

    /// Rotate the bottom side of the cube.
    pub fn rotate_bottom(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::NegZ,
            affected_range: layer.into(),
        })
    }

    /// Rotate the left side of the cube.
    pub fn rotate_left(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::NegY,
            affected_range: layer.into(),
        })
    }

    /// Rotate the right side of the cube.
    pub fn rotate_right(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::Y,
            affected_range: layer.into(),
        })
    }

    /// Rotate the front side of the cube.
    pub fn rotate_front(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::X,
            affected_range: layer.into(),
        })
    }

    /// Rotate the back side of the cube.
    pub fn rotate_back(layer: Layer, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: AxisInner::NegX,
            affected_range: layer.into(),
        })
    }

    /// Rotate the whole cube around an axis.
    pub fn rotate_cube(axis: Axis, move_type: MoveType) -> Result<Self, Error> {
        Ok(Move {
            move_type,
            axis: axis.into(),
            affected_range: LayerInner::WholeCube,
        })
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
