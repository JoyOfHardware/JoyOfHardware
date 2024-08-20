use zoon::*;
use zoon::eprintln;
use std::rc::Rc;

mod page;
mod router;
mod theme;
mod assets;

pub static WINDOW_SIZE: Lazy<Mutable<u32>> = Lazy::new(|| Mutable::new(0));

fn main() {
    start_app("app", page::root);
}
