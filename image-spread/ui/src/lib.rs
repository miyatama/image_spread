use clap::{Parser, Subcommand};
use infra_handler::InfraHandlerImpl;
use repository_handler::RepositoryHandlerImpl;
use std::cmp::min;
use tracing::{Level, event};
use usecase::{ParseImageCase};
use usecase_handler::{UsecaseHandler, UsecaseHandlerImpl};
use util::AppResult;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    ImageInfo {
        #[clap(short = 'w', long = "width", required = true, ignore_case = true)]
        grid_width: u32,
        #[clap(short = 'p', long = "path", required = true, ignore_case = true)]
        path: String,
    },
}

#[tracing::instrument]
pub async fn run(config: &Config) -> AppResult<()> {
    let infra_hanler = InfraHandlerImpl::new();
    match infra_hanler {
        Err(e) => {
            return Err(e);
        }
        _ => {}
    }
    let infra_hanler = infra_hanler.unwrap();
    let repository_handler = RepositoryHandlerImpl::new(&infra_hanler);
    let usecases = UsecaseHandlerImpl::new(&repository_handler).await;
    match &config.subcommand {
        SubCommands::ImageInfo { grid_width, path } => {
            let usecase = usecases.parse_image();
            match usecase.run() {
                Ok(image_info) => {
                    // TODO ここでimage_infoのjsonを出力
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

pub fn get_args() -> AppResult<Config> {
    let args = Config::parse();
    Ok(args)
}
