use cargo_toml::Manifest;
use clap::Parser;

pub mod style_check;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(short, long, default_value = "Cargo.toml")]
    pub manifest_path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // Don't do anything for now
    let _ = Manifest::from_path(&args.manifest_path)?;

    let path = std::path::Path::new(&args.manifest_path);
    let project_directory = path.parent().unwrap();
    // recursively find all rust files in the project directory
    let rust_files = walkdir::WalkDir::new(project_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |e| e == "rs"))
        .filter_map(|entry| {
            if entry.file_type().is_file() {
                Some(entry.path().to_path_buf())
            } else {
                None
            }
        });

    for file in rust_files {
        style_check::check_files(file)?;
    }

    Ok(())
}
