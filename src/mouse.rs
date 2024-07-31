use enigo::{Coordinate, Enigo, InputError, Mouse};

use crate::point::Point;

/// Makes it easier to work with enigo crate
pub struct Cursor {
    pub enigo: Enigo,
}

impl Cursor {
    pub fn new(enigo: Enigo) -> Self {
        Self { enigo }
    }

    pub fn move_mouse(&mut self, point: Point) -> Result<(), InputError> {
        let (w, h) = self.enigo.main_display()?;
        let new_point = point.scale_point_to_screen(w, h);

        self.enigo
            .move_mouse(new_point.x, new_point.y, Coordinate::Abs)?;
        Ok(())
    }

    pub fn move_mouse_relative(&mut self, point: Point) -> Result<(), InputError> {
        self.enigo.move_mouse(point.x, point.y, Coordinate::Rel)?;
        Ok(())
    }
}
