//! A Piston library for handling split state and events.

extern crate input;
extern crate vecmath;

use input::{Button, GenericEvent, MouseButton};

use self::math::{is_inside, inside_pos, Matrix2d, Rectangle};

mod math;

const LEFT: u8 = 0x1;
const RIGHT: u8 = 0x2;
const TOP: u8 = 0x4;
const BOTTOM: u8 = 0x8;

/// Stores split layout settings.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SplitLayoutSettings {
    /// The border width.
    pub border: f64,
    /// The minimum size of center.
    pub center_min_size: [f64; 2],
    /// The initial value for left split.
    pub left_value: f64,
    /// The minimum value for left split.
    pub left_min_value: f64,
    /// The initial value for right split.
    pub right_value: f64,
    /// The minimum value for right split.
    pub right_min_value: f64,
    /// The initial value for top split.
    pub top_value: f64,
    /// The minimum value for top split.
    pub top_min_value: f64,
    /// The initial value for bottom split.
    pub bottom_value: f64,
    /// The minimum value for bottom split.
    pub bottom_min_value: f64,
    /// Locks left split.
    pub lock_left: bool,
    /// Locks right split.
    pub lock_right: bool,
    /// Locks top split.
    pub lock_top: bool,
    /// Locks bottom split.
    pub lock_bottom: bool,
}

impl SplitLayoutSettings {
    /// Creates a new `SplitLayoutSettings` object with values set to minimum.
    ///
    /// Work area minimum size is set to 1x1.
    pub fn new(border: f64, min_value: f64) -> SplitLayoutSettings {
        SplitLayoutSettings {
            border: border,
            center_min_size: [1.0; 2],
            left_value: min_value,
            left_min_value: min_value,
            right_value: min_value,
            right_min_value: min_value,
            top_value: min_value,
            top_min_value: min_value,
            bottom_value: min_value,
            bottom_min_value: min_value,
            lock_left: false,
            lock_right: false,
            lock_top: false,
            lock_bottom: false,
        }
    }

    /// Set the value and minimum value for left split.
    pub fn left(mut self, value: f64, min_value: f64) -> SplitLayoutSettings {
        self.left_value = value;
        self.left_min_value = min_value;
        self
    }

    /// Sets the value and minimum value for right split.
    pub fn right(mut self, value: f64, min_value: f64) -> SplitLayoutSettings {
        self.right_value = value;
        self.right_min_value = min_value;
        self
    }

    /// Sets the value and minimum value for top split.
    pub fn top(mut self, value: f64, min_value: f64) -> SplitLayoutSettings {
        self.top_value = value;
        self.top_min_value = min_value;
        self
    }

    /// Sets the value and minimum value for bottom split.
    pub fn bottom(mut self, value: f64, min_value: f64) -> SplitLayoutSettings {
        self.bottom_value = value;
        self.bottom_min_value = min_value;
        self
    }

    /// Locks left split, sets the minimum value at the same time.
    pub fn lock_left(mut self, value: f64) -> SplitLayoutSettings {
        self.lock_left = true;
        self.left_value = value;
        self.left_min_value = value;
        self
    }

    /// Locks right split, sets the minimum value at the same time.
    pub fn lock_right(mut self, value: f64) -> SplitLayoutSettings {
        self.lock_right = true;
        self.right_value = value;
        self.right_min_value = value;
        self
    }

    /// Locks top split, sets the minimum value at the same time.
    pub fn lock_top(mut self, value: f64) -> SplitLayoutSettings {
        self.lock_top = true;
        self.top_value = value;
        self.top_min_value = value;
        self
    }

    /// Locks bottom split, sets the minimum value at the same time.
    pub fn lock_bottom(mut self, value: f64) -> SplitLayoutSettings {
        self.lock_bottom = true;
        self.bottom_value = value;
        self.bottom_min_value = value;
        self
    }
}

/// Stores information about split layout.
///
/// The layout is split into left, right, top and bottom panel.
pub struct SplitLayoutController {
    /// The left split controller.
    pub left: SplitController,
    /// The right split controller.
    pub right: SplitController,
    /// The top split controller.
    pub top: SplitController,
    /// The bottom split controller.
    pub bottom: SplitController,
    // Center minimum size.
    center_min_size: [f64; 2],
    // Which splits are dragged.
    drag_splits: u8,
    // Which splits are locked.
    lock_splits: u8,
}

impl SplitLayoutController {
    /// Creates a new `SplitLayoutController`.
    pub fn new(settings: &SplitLayoutSettings) -> SplitLayoutController {
        SplitLayoutController {
            left: SplitController::new(settings.left_value, settings.left_min_value,
                                       settings.border, SplitOrientation::Left),
            right: SplitController::new(settings.right_value, settings.right_min_value,
                                        settings.border, SplitOrientation::Right),
            top: SplitController::new(settings.top_value, settings.top_min_value,
                                      settings.border, SplitOrientation::Top),
            bottom: SplitController::new(settings.bottom_value, settings.bottom_min_value,
                                         settings.border, SplitOrientation::Bottom),
            center_min_size: settings.center_min_size,
            drag_splits: 0,
            lock_splits: if settings.lock_left {LEFT} else {0} |
                         if settings.lock_right {RIGHT} else {0} |
                         if settings.lock_top {TOP} else {0} |
                         if settings.lock_bottom {BOTTOM} else {0},
        }
    }

    /// Handles event.
    pub fn event<E: GenericEvent>(&mut self, rect: Rectangle, transform: Matrix2d, e: &E) {
        let bounds = self.bounds(rect);

        if (self.lock_splits & TOP) != TOP {
            if self.drag_splits == 0 || (self.drag_splits & TOP) == TOP {
                let layout = self.top_bottom_layout();
                let max_value = bounds[3] - self.bottom.value - self.center_min_size[1] -
                          self.top.border - self.bottom.border;
                self.top.event(layout, max_value, bounds, transform, e);
            }
        }
        if (self.lock_splits & BOTTOM) != BOTTOM {
            if self.drag_splits == 0 || (self.drag_splits & BOTTOM) == BOTTOM {
                let layout = self.top_bottom_layout();
                let max_value = bounds[3] - self.top.value - self.center_min_size[1] -
                                self.bottom.border - self.top.border;
                self.bottom.event(layout, max_value, bounds, transform, e);
            }
        }
        if (self.lock_splits & LEFT) != LEFT {
            if self.drag_splits == 0 || (self.drag_splits & LEFT) == LEFT {
                let layout = self.left_right_layout(SplitLayoutPurpose::Event);
                let max_value = bounds[2] - self.right.value - self.center_min_size[0] -
                                self.left.border - self.right.border;
                self.left.event(layout, max_value, bounds, transform, e);
            }
        }
        if (self.lock_splits & RIGHT) != RIGHT {
            if self.drag_splits == 0 || (self.drag_splits & RIGHT) == RIGHT {
                let layout = self.left_right_layout(SplitLayoutPurpose::Event);
                let max_value = bounds[2] - self.left.value - self.center_min_size[0] -
                                self.right.border - self.left.border;
                self.right.event(layout, max_value, bounds, transform, e);
            }
        }

        self.drag_splits = if self.top.is_dragging() {TOP} else {0} |
                          if self.bottom.is_dragging() {BOTTOM} else {0} |
                          if self.left.is_dragging() {LEFT} else {0} |
                          if self.right.is_dragging() {RIGHT} else {0};
    }

    /// Returns the left/right split layout.
    ///
    /// The left/right split layout depends on whether your purpose is to draw something or
    /// handle events. When handling events, the rectangle overlaps with the top and bottom split.
    pub fn left_right_layout(&self, purpose: SplitLayoutPurpose) -> SplitLayout {
        let sign = purpose.sign();
        SplitLayout {
            start: self.top.value + sign * self.top.border,
            end: self.bottom.value + sign * self.bottom.border
        }
    }

    /// Returns the top/bottom split layout.
    pub fn top_bottom_layout(&self) -> SplitLayout {
        SplitLayout {start: 0.0, end: 0.0}
    }

    /// Computes split rectangles for drawing `[left, right, top, bottom]`.
    pub fn rectangles(&self, rect: Rectangle) -> [Rectangle; 4] {
        let bounds = self.bounds(rect);
        let top_bottom_layout = self.top_bottom_layout();
        let left_right_layout = self.left_right_layout(SplitLayoutPurpose::Draw);
        [
            self.left.line_rect(left_right_layout, bounds),
            self.right.line_rect(left_right_layout, bounds),
            self.top.line_rect(top_bottom_layout, bounds),
            self.bottom.line_rect(top_bottom_layout, bounds),
        ]
    }

    /// Returns the split controller states `[left, right, top, bottom]`.
    pub fn states(&self) -> [SplitState; 4] {
        [self.left.state(), self.right.state(), self.top.state(), self.bottom.state()]
    }

    /// Computes panel rectangles for layout `[left, right, top, bottom, center]`.
    pub fn panel_rectangles(&self, rect: Rectangle) -> [Rectangle; 5] {
        let bounds = self.bounds(rect);
        let left_right_y = bounds[1] + self.top.value + self.top.border;
        let left_right_h = bounds[3] - self.top.value - self.top.border -
                           self.bottom.value - self.bottom.border;
        [
            [bounds[0], left_right_y, self.left.value, left_right_h],
            [bounds[0] + bounds[2] - self.right.value, left_right_y,
             self.right.value, left_right_h],
            [bounds[0], bounds[1], bounds[2], self.top.value],
            [bounds[0], bounds[1] + bounds[3] - self.bottom.value, bounds[2], self.bottom.value],
            [bounds[0] + self.left.value + self.left.border, left_right_y,
             bounds[2] - self.right.value - self.right.border -
             self.left.value - self.left.border, left_right_h],
        ]
    }

    /// Computes the minimum size using current values in split controls.
    ///
    /// The current values in the split controls are used instead of the minimum values,
    /// because the splits should appear visually with the same current value.
    pub fn min_size(&self) -> [f64; 2] {
        [
            self.left.value + self.left.border + self.right.value + self.right.border +
            self.center_min_size[0],
            self.top.value + self.top.border + self.bottom.value + self.bottom.border +
            self.center_min_size[1]
        ]
    }

    /// Computes the bounds from window bounds `[x, y, w, h]`.
    ///
    /// Does not get less in size than specified by `min_size`.
    pub fn bounds(&self, rect: Rectangle) -> Rectangle {
        let min_size = self.min_size();
        [rect[0], rect[1], rect[2].max(min_size[0]), rect[3].max(min_size[1])]
    }
}

/// Stores information about an UI split.
pub struct SplitController {
    /// Whether the mouse is hovering over the split.
    mouse_hover: bool,
    /// Whether the user is dragging the split.
    dragging: bool,
    /// The value of split.
    pub value: f64,
    /// The minimum value of split.
    pub min_value: f64,
    /// The border width.
    pub border: f64,
    /// The orientation of split.
    pub orientation: SplitOrientation,
}

impl SplitController {
    /// Creates a new `SplitController`.
    pub fn new(
        value: f64,
        min_value: f64,
        border: f64,
        orientation: SplitOrientation
    ) -> SplitController {
        SplitController {
            mouse_hover: false,
            dragging: false,
            value: value,
            min_value: min_value,
            border: border,
            orientation: orientation,
        }
    }

    /// Gets whether the split is currently being dragged by the user.
    pub fn is_dragging(&self) -> bool {self.dragging}

    /// Handles event.
    pub fn event<E: GenericEvent>(
        &mut self,
        layout: SplitLayout,
        max_value: f64,
        rect: Rectangle,
        transform: Matrix2d,
        e: &E
    ) {
        if let Some(pos) = e.mouse_cursor_args() {
            let pos = inside_pos(pos, transform);
            if self.dragging {
                match self.orientation {
                    SplitOrientation::Left => {
                        self.value = (pos[0] - rect[0] - 0.5 * self.border)
                            .max(self.min_value)
                            .min(max_value);
                    }
                    SplitOrientation::Right => {
                        self.value = (rect[2] - pos[0] + rect[0] - 0.5 * self.border)
                            .max(self.min_value)
                            .min(max_value);
                    }
                    SplitOrientation::Top => {
                        self.value = (pos[1] - rect[1] - 0.5 * self.border)
                            .max(self.min_value)
                            .min(max_value);
                    }
                    SplitOrientation::Bottom => {
                        self.value = (rect[1] + rect[3] - pos[1] - 0.5 * self.border)
                            .max(self.min_value)
                            .min(max_value);
                    }
                }
            }
            let line_rect = self.line_rect(layout, rect);
            self.mouse_hover = is_inside(pos, line_rect);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if self.mouse_hover {
                self.dragging = true;
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            self.dragging = false;
        }
    }

    /// Gets the current state of split.
    pub fn state(&self) -> SplitState {
        match (self.mouse_hover, self.dragging) {
            (false, false) => SplitState::Inactive,
            (true, false) => SplitState::Hover,
            (true, true) => SplitState::Drag,
            (false, true) => SplitState::DragNotFollowing,
        }
    }

    /// Gets line rectangle `[x, y, w, h]` from rectangle `[x, y, w, h]` of parent panel.
    pub fn line_rect(&self, layout: SplitLayout, rect: Rectangle) -> Rectangle {
        match self.orientation {
            SplitOrientation::Left => {
                [rect[0] + self.value, rect[1] + layout.start,
                 self.border, rect[3] - layout.start - layout.end]
            }
            SplitOrientation::Right => {
                [rect[0] + rect[2] - self.value - self.border, rect[1] + layout.start,
                 self.border, rect[3] - layout.start - layout.end]
            }
            SplitOrientation::Top => {
                [rect[0] + layout.start, rect[1] + self.value,
                 rect[2] - layout.start - layout.end, self.border]
            }
            SplitOrientation::Bottom => {
                [rect[0] + layout.start, rect[1] + rect[3] - self.value - self.border,
                 rect[2] - layout.start - layout.end, self.border]
            }
        }
    }
}

/// Stores split layout.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SplitLayout {
    /// The start of split from edge of parent panel.
    pub start: f64,
    /// The end of split from edge of parent panel.
    pub end: f64,
}

/// Stores split layout purpose.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SplitLayoutPurpose {
    /// For drawing.
    Draw,
    /// For handling events.
    Event,
}

impl SplitLayoutPurpose {
    fn sign(self) -> f64 {if let SplitLayoutPurpose::Draw = self {1.0} else {0.0}}
}

/// Orients split from an edge of parent panel.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SplitOrientation {
    /// Splits from left edge of parent panel.
    Left,
    /// Splits from right edge of parent panel.
    Right,
    /// Splits from top edge of parent panel.
    Top,
    /// Splits from bottom edge of parent panel.
    Bottom,
}

/// Gets the state of split.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SplitState {
    /// Split is inactive.
    Inactive,
    /// Mouse cursor is hovering above split.
    Hover,
    /// User is dragging the split.
    Drag,
    /// User is dragging, but split is not following.
    DragNotFollowing,
}
