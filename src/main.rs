mod bangs;
mod cli;
mod data_io;

use anyhow::{Ok, Result};
use bangs::BangMap;
use cli::Cli;
use data_io::DataIO;
use log::{error, info};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let data = DataIO::new()?;
    let args = Cli::init();

    if args.clean && args.rebuild {
        error!("❌ Cannot use --build and --clean together.");
        std::process::exit(1);
    }

    if args.clean {
        return data.clean_data_dir();
    }

    if args.rebuild {
        let _ = data.clean_data_dir();
        return data.build_bangs().await;
    }

    if !data.bin_dir.exists() {
        info!("❌ Binary map does not exist, building it.");
        let _ = data.build_bangs().await;
    }

    let raw_bangs: HashMap<String, bangs::Bang> = data.read_bangs_binary()?;
    let bangs = BangMap::new(raw_bangs);

    if args.list {
        return bangs.list_keys();
    }

    if let Some(tag) = &args.tag {
        if tag.starts_with('!') {
            let url = bangs.resolve_bang(tag, args.query)?;
            println!("{}", url);
        } else {
            error!("❌ Tag must start with the bang operator '!'");
            std::process::exit(1);
        }
    }
    Ok(())
}
