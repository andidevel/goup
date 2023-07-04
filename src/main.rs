use std::env;
use std::path;

use clap::Parser;

mod req;
mod cmd;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
struct Cli {
    #[arg(short='b', long)]
    gobin: Option<path::PathBuf>,
    #[arg(short, long)]
    temp: Option<path::PathBuf>,
}

fn get_new_version() -> Result<req::VersionItem, Box<dyn std::error::Error>> {
    println!("Checking last Golang version...");
    let go_response = req::get()?;
    // Usually, the first item is the latest version available
    if go_response.len() > 0 {
       return Ok(go_response.get(0).cloned().unwrap());
    }
    Err(
        Box::new(
            cmd::GoError {
                message: String::from("Golang API return an empty version list")
            }
        )
    )
}

fn main() {
    let target_os = "linux";
    let target_arch = "amd64";

    println!("goup version {}\n", VERSION);

    let args = Cli::parse();

    let last_go_version_remote = match get_new_version() {
        Ok(version) => version,
        Err(error) => panic!("Error getting remote version: {:?}", error),
    };
    let last_go_version: cmd::GoVersion = cmd::GoVersion::new(last_go_version_remote.version);

    let go_cmd = cmd::GoCommand::new(args.gobin.unwrap_or(path::PathBuf::from("/usr/local/go/bin/")));
    let local_go_version = match go_cmd.version() {
        Ok(version) => version,
        Err(error) => panic!("Error getting local version: {:?}", error),
    };

    let local_go_install = match go_cmd.root() {
        Ok(path) => path,
        Err(error) => panic!("Error getting path installation: {:?}", error),
    };
    let go_install_to = path::Path::new(local_go_install.as_str()).parent().unwrap().to_str().unwrap();

    println!("  Last version: [{}]", last_go_version.version);
    println!(" Local version: [{}]", local_go_version.version);
    println!("Installed path: [{}]", local_go_install);
    println!("    Install to: [{}]", go_install_to);

    let path_to_download = args.temp.unwrap_or(env::temp_dir());
    if last_go_version.gt(&local_go_version) {
        println!("Last version is greater than local version...");
        for f in last_go_version_remote.files {
            if f.os.as_str() == target_os && f.arch.as_str() == target_arch && f.kind == "archive" {
                println!("Downloading file: {}", f.filename);
                let version_file: req::FileDownload = req::FileDownload::new(path_to_download.to_str().unwrap(), &f.filename);
                if !path::Path::new(version_file.download_to.as_str()).exists() {
                    version_file.download();
                    println!("File downloaded...");
                }
                else {
                    println!("[CACHE] File already downloaded ({})", version_file.download_to);
                }
                println!("Installing...");
                match cmd::remove_dir(local_go_install.as_str()) {
                    Ok(()) => (),
                    Err(error) => println!(" -> Error removing `{}`: {}", local_go_install, error.to_string()),
                };
                
                println!("Unpacking `{}`", version_file.download_to.as_str());
                match cmd::go_install(go_install_to, version_file.download_to.as_str()) {
                    Ok(output) => println!("{}", output),
                    Err(error) => panic!("Error installing: {}", error.to_string()),
                };
                println!("Verifying installation...");
                let installed_go_version = match go_cmd.version() {
                    Ok(version) => version,
                    Err(error) => panic!("Error verifying installation: {:?}", error),
                };
                if installed_go_version.eq(&last_go_version) {
                    println!("Go version {} installed!", installed_go_version.version);
                }
                else {
                    println!("Problem: Installed version ({}) is differnt from last available version ({})", installed_go_version.version, last_go_version.version);
                }
                break;            
            }
        }
    }

}
