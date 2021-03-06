use bindings::*;
use {wrap_safe_type, Canvas, Matrix, Picture, Rect};

pub struct Drawable {
  pub(crate) raw_pointer: *mut sk_drawable_t,
}

impl Drop for Drawable {
  fn drop(&mut self) {
    unsafe { sk_drawable_unref(self.raw_pointer) };
  }
}

impl Drawable {
  pub fn get_generation_id(&mut self) -> u32 {
    unsafe { sk_drawable_get_generation_id(self.raw_pointer) }
  }

  pub fn get_bounds(&mut self) -> Rect {
    let mut rect = Rect {
      left: 0.,
      right: 0.,
      top: 0.,
      bottom: 0.,
    };
    unsafe {
      sk_drawable_get_bounds(self.raw_pointer, &mut rect);
    }
    rect
  }

  pub fn draw(&mut self, canvas: &Canvas, matrix: Option<&Matrix>) {
    unsafe {
      sk_drawable_draw(
        self.raw_pointer,
        canvas.raw_pointer,
        if let Some(m) = matrix {
          m
        } else {
          std::ptr::null()
        },
      );
    }
  }

  pub fn new_picture_snapshot(&mut self) -> Picture {
    wrap_safe_type! {
      Picture <= sk_drawable_new_picture_snapshot(self.raw_pointer)
    }
  }

  pub fn notify_drawing_changed(&mut self) {
    unsafe {
      sk_drawable_notify_drawing_changed(self.raw_pointer);
    }
  }
}
