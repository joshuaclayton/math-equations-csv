use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "math",
    about = "A command line tool to generate math",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
struct Flags {
    /// Minimum value
    #[structopt(long, default_value = "0")]
    pub minimum: usize,

    /// Maximum value
    #[structopt(long, default_value = "100")]
    pub maximum: usize,

    /// Operations
    #[structopt(long, possible_values = &["addition", "subtraction", "multiplication", "division"], default_value = "addition", case_insensitive = true)]
    pub operation: Vec<Operation>,
}

#[derive(Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operation {
    fn run(&self, left: usize, right: usize) -> Option<String> {
        match self {
            Operation::Addition => Some(format!("{}+{},{}", left, right, left + right)),
            Operation::Subtraction => Some(format!("{}-{},{}", left + right, right, left)),
            Operation::Multiplication => Some(format!("{}x{},{}", left, right, left * right)),
            Operation::Division => {
                if right == 0 {
                    None
                } else if left == 0 {
                    Some(format!("{}÷{},{}", left, right, 0))
                } else {
                    Some(format!("{}÷{},{}", left * right, right, left))
                }
            }
        }
    }
}

impl std::str::FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "addition" => Ok(Operation::Addition),
            "subtraction" => Ok(Operation::Subtraction),
            "multiplication" => Ok(Operation::Multiplication),
            "division" => Ok(Operation::Division),
            v => Err(format!("Unknown operation: {}", v)),
        }
    }
}

fn main() {
    let flags = Flags::from_args();
    for left in flags.minimum..=flags.maximum {
        for right in flags.minimum..=flags.maximum {
            for op in &flags.operation {
                if let Some(result) = op.run(left, right) {
                    println!("{}", result)
                }
            }
        }
    }
}
