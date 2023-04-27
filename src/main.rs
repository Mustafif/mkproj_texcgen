mod builder;
#[allow(dead_code)]
mod generated;
mod template;

use crate::builder::{generate, generate_all, get_templates, save, Builder};
use std::path::PathBuf;
use structopt::StructOpt;
use texcore::template::Version;
use texcreate_repo::release::Release;
use texcreate_repo::Repo;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Result};

#[derive(StructOpt)]
#[structopt(about = "A template generator for the TexCreate project")]
enum CLI {
    #[structopt(about = "Initialize Output Directory Structure")]
    Init,
    #[structopt(about = "Generate template")]
    Gen {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Generate all templates in `src/generated`")]
    GenAll {
        #[structopt(short, long)]
        level: Option<u8>,
    },
    #[structopt(about = "Saves template to TexCreate custom directory")]
    Save {
        #[structopt(short, long)]
        name: String,
    },
    #[structopt(about = "Creates a new repo file given the current templates")]
    Release {
        #[structopt(short, long)]
        version: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::from_args();

    match cli {
        CLI::Init => Builder::default().create_dirs().await?,
        CLI::Gen { level } => {
            let builder = match level {
                None => Builder::default(),
                Some(l) => Builder::new(l),
            };
            generate(builder).await?
        }
        CLI::GenAll { level } => {
            let builder = match level {
                None => Builder::default(),
                Some(l) => Builder::new(l),
            };
            generate_all(builder).await?
        }
        CLI::Save { name } => save(&name).await?,
        CLI::Release { version } => {
            // get texcreate min version
            println!("Enter TexCreate Minimum Version: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let texc_version= str_to_version(input.trim());

            let templates = get_templates().await;
            //println!("{:?}", &templates);
            let repo = Repo::new(version, &templates, texc_version);
            let mut file = File::create("repo.toml").await?;
            let content = repo.to_string();
            file.write_all(content.as_bytes()).await?;
            let release = Release::new(PathBuf::from("release.toml")).await?;
            release
                .build_release(PathBuf::from("release_notes"))
                .await?;
        }
    }

    Ok(())
}

// temporary fix:
// Next issue to put a builder config to automate release process.
fn str_to_version(s: &str) -> Version{
    let split: Vec<&str> = s.rsplit('.').collect();
    Version::new(split[2].parse().unwrap(), split[1].parse().unwrap(), split[0].parse().unwrap())
}
