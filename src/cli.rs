use clap::Parser;
use env_logger;

#[derive(Parser, Debug)]
#[command(name = "banger", author, version, about)]
pub struct Cli {
    /// Clean the data directory
    #[arg(long)]
    pub clean: bool,

    /// Suppress all output except final result
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Fetches bangs fron duckduckgo and builds a binary hashmap
    #[arg(long)]
    pub rebuild: bool,

    /// The bang tag (e.g., "g" for Google)
    #[arg(required = false)]
    pub tag: Option<String>,

    // Define query as a required positional argument, allowing multiple words
    /// The search query
    #[arg(requires = "tag")]
    pub query: Option<Vec<String>>,
}

impl Cli {
    pub fn init() -> Self {
        let cli = Cli::parse();
        cli.init_logger();
        cli
    }

    fn init_logger(&self) {
        env_logger::Builder::new()
            .filter_level(self.verbose.into())
            .init();
    }
}
