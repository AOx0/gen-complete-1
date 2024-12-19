mod codegen;
mod constants;
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

pub struct GenBuilder {
    cli: Command,
}

pub struct Gen {
    cli: std::rc::Rc<Command>,
}

impl GenBuilder {
    pub fn new(name: &'static str, description: &'static str) -> GenBuilder {
        GenBuilder {
            cli: Command {
                name,
                description,
                include_in_codegen: false,
                ..Default::default()
            },
        }
    }

    pub fn add_flag(self, flag: (char, &'static str, &'static str)) -> Self {
        self.cli.flags.borrow_mut().push(flag);
        self
    }

    pub fn add_flags(self, flags: &[(char, &'static str, &'static str)]) -> Self {
        self.cli.flags.borrow_mut().extend_from_slice(flags);
        self
    }

    pub fn add_command(self, command: std::rc::Rc<Command>) -> Self {
        self.cli.children.borrow_mut().push(command);
        self
    }

    pub fn add_commands(self, commands: &[std::rc::Rc<Command>]) -> Self {
        self.cli.children.borrow_mut().extend_from_slice(commands);
        self
    }

    pub fn build(self) -> Gen {
        let cli = self.cli.to_rc();
        cli.set_children_parents();
        cli.setup_args();

        Gen { cli }
    }
}

impl Codegen for Gen {
    fn generate(&self) -> String {
        let mut res = String::new();

        res += sec!("Fish functions");
        res += constants::FISH_FUNCTIONS;
        res += &format!("\ncomplete -c {name} -f\n", name = self.cli.name);

        res + &self.cli.generate()
    }
}
