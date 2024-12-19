mod codegen;
mod tree;

pub use codegen::Codegen;
pub use tree::{Args, Command};

#[macro_export]
macro_rules! sec {
    ($section:expr) => {
        concat!(sec!(), "# ", $section, "\n")
    };
    () => {
        "\n\n"
    };
}
