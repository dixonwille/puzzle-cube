use std::ops::Range;

/// Represents a possible move on a cube.
pub struct Move {
    inner: MoveInner,
}

// QUESTION: Should a move be built based on the cube it is used for,
// this will allow us to validate the range! But may have some funky API for usage.

/// Possible moves to make on a cube. Each move has a bool that specifies whether
/// the move should be counter clockwise or not.
enum MoveInner {
    /// Rotate the Top layers. The boolean is to sepcify to move counter-clockwise.
    Top(Layer, bool),

    /// Rotate the Bottom layers. The boolean is to sepcify to move counter-clockwise.
    Bottom(Layer, bool),

    /// Rotate the Left layers. The boolean is to sepcify to move counter-clockwise.
    Left(Layer, bool),

    /// Rotate the Right layers. The boolean is to sepcify to move counter-clockwise.
    Right(Layer, bool),

    /// Rotate the Front layers. The boolean is to sepcify to move counter-clockwise.
    Front(Layer, bool),

    /// Rotate the Back layers. The boolean is to sepcify to move counter-clockwise.
    Back(Layer, bool),

    /// Rotate the full Cube around an axis. The boolean is to specify to move counter-clockwise.   
    Cube(Axis, bool),
}

/// Layer describes how you are selecting which layer to apply a Move to.
enum Layer {
    /// Select a single layer (0 indexed)
    Single(usize),
    /// Select a range of layers to change (0 indexed inclusive below exclusizve above)
    Many(Range<usize>),
}

/// Describes the Axis of rotation for the Cube
enum Axis {
    /// Rotate Cube around the X-Axis
    X,
    /// Rotate Cube around the Y-Axis
    Y,
    /// Rotate Cube around the Z-Axis
    Z,
}
