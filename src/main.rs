use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "bang-builder", author, version, about)]
struct CliArgs {
    #[clap(subcommand)]
    command: Option<Command>,

    // Argument for the default command
    #[arg(index = 1)]
    tag: Option<String>,

    // Optional argument for the search query in the default command
    #[arg(index = 2)]
    query: Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    Generate {
        #[arg(short, long, default_value = "assets/bangs.json")]
        output: String,
    },
    Build {
        #[arg(short, long, default_value = "assets/bangs.json")]
        input: String,

        #[arg(short, long, default_value = "assets/bangs.bin")]
        output: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Bang {
    #[serde(default)]
    c: Option<String>, // Category
    d: String, // Direct Link
    #[serde(default)]
    r: Option<Number>,
    #[serde(default)]
    sc: Option<String>, // Secondary Category
    t: String,
    u: String, // URL template
}

async fn generate_bangs(output_path: &str) -> Result<()> {
    println!("Fetching data from duckduckgo.com/bangs.js...");
    let response = reqwest::get("https://duckduckgo.com/bang.js").await?;
    let text = response.text().await?;

    println!("Parsing JSON data...");
    let bangs: Vec<Bang> = serde_json::from_str(&text)?;

    let output_path = Path::new(output_path);
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    println!(
        "Saving {} bangs to {}...",
        bangs.len(),
        output_path.display()
    );
    let mut file = fs::File::create(output_path)?;
    let json_output = serde_json::to_string_pretty(&bangs)?;
    file.write_all(json_output.as_bytes())?;

    println!("✅ Successfully generated and saved bangs data.");

    Ok(())
}

// Function to get the template by tag from the in-memory HashMap
fn get_template_by_tag(map: &HashMap<String, String>, tag: &str) -> Result<String> {
    map.get(tag)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Tag '{}' not found", tag))
}

fn process_bangs(input_path: &str, output_path: &str) -> Result<()> {
    let json = fs::read_to_string(input_path)?;
    let raw: Vec<serde_json::Value> = serde_json::from_str(&json)?;

    let mut map = HashMap::new();
    for item in raw {
        if let (Some(t), Some(u)) = (item.get("t"), item.get("u")) {
            map.insert(
                t.as_str().unwrap().to_string(),
                u.as_str().unwrap().to_string(),
            );
        }
    }

    let bytes = bincode::serialize(&map)?;
    fs::write(output_path, bytes)?;
    println!("✅ Wrote {} bangs to {}", map.len(), output_path);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Load and deserialize the binary map once at the start
    let bytes = fs::read("assets/bangs.bin")?;
    let bangs_map: HashMap<String, String> = bincode::deserialize(&bytes)?;

    match args.command {
        Some(Command::Generate { output }) => {
            generate_bangs(&output).await?;
        }
        Some(Command::Build { input, output }) => {
            process_bangs(&input, &output)?;
        }
        None => {
            // Default command: get template by tag and optional query
            if let Some(tag) = args.tag {
                match get_template_by_tag(&bangs_map, &tag) {
                    Ok(mut template) => {
                        // Use mut because we might modify it
                        if let Some(query) = args.query {
                            // URL encode the query
                            let encoded_query = urlencoding::encode(&query);
                            // Replace {{{s}}} with the encoded query
                            template = template.replace("{{{s}}}", &encoded_query);
                        }
                        println!("{}", template);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                // Neither command nor tag was provided, print a usage message
                eprintln!("Usage: bang-builder <tag> [query] or bang-builder <command>");
                eprintln!("Commands: generate, build");
            }
        }
    }

    Ok(())
}
