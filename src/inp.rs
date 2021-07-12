use piston_window::Key;

#[allow(dead_code)]
pub enum InputReceived {
    Mouse(bool, (f64, f64)),
    Scroll(f64, (f64, f64)),
    Keyboard(Key),
}
