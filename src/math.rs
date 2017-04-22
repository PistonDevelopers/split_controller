use vecmath;
use vecmath::mat2x3_inv as inv;
use vecmath::row_mat2x3_transform_pos2 as transform_pos;

/// The type used for scalars.
pub type Scalar = f64;

/// The type used for matrices.
pub type Matrix2d<T = Scalar> = vecmath::Matrix2x3<T>;

/// Rectangle dimensions: [x, y, w, h]
pub type Rectangle<T = Scalar> = [T; 4];

/// The type used for 2D vectors.
pub type Vec2d<T = Scalar> = vecmath::Vector2<T>;

/// Returns true if point is inside rectangle.
pub fn is_inside(pos: Vec2d, rect: Rectangle) -> bool {
    pos[0] >= rect[0] && pos[1] >= rect[1] && pos[0] < rect[0] + rect[2] &&
    pos[1] < rect[1] + rect[3]
}

/// Returns the position inside a transform matrix.
pub fn inside_pos(outside_pos: Vec2d, transform: Matrix2d) -> Vec2d {
    let inv = inv(transform);
    transform_pos(inv, outside_pos)
}
