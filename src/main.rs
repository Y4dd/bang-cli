mod bangs;
mod cli;
mod data_io;

use anyhow::Ok;
use bangs::BangMap;
use cli::Cli;
use data_io::DataIO;
use log::error;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        let _ = data.build_bangs().await;
    }

    if let Some(tag) = &args.tag {
        let raw_bangs: HashMap<String, bangs::Bang> = data.read_bangs_binary()?;
        let bangs = BangMap::new(raw_bangs);
        let url = bangs.resolve_bang(tag, args.query)?;
        println!("{}", url);
    }
    Ok(())
}
