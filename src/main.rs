use std::env;
use std::path;

use clap::Parser;

use goup::{
    req,
    cmd,
    get_new_version,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
struct Cli {
    #[arg(short='p', long="path")]
    go_bin_path: Option<path::PathBuf>,
    #[arg(short='t', long="temp")]
    temp_dir: Option<path::PathBuf>,
}

fn main() {
    println!("goup version {}\n", VERSION);

    // TODO: get info from host machine
    let target_os = cmd::os_type().unwrap_or("unknown".to_string());
    let target_arch = cmd::os_arch();
    println!("OS: {} Arch: {}", target_os, target_arch);

    let args = Cli::parse();

    let last_go_version_remote = match get_new_version() {
        Ok(version) => version,
        Err(error) => panic!("Error getting remote version: {:?}", error),
    };
    let last_go_version: cmd::GoVersion = cmd::GoVersion::new(last_go_version_remote.version);

    let go_cmd = cmd::GoCommand::new(args.go_bin_path.unwrap_or(path::PathBuf::from("/usr/local/go/bin/")));
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

    let path_to_download = args.temp_dir.unwrap_or(env::temp_dir());
    if last_go_version.gt(&local_go_version) {
        println!("Last version is greater than local version...");
        for f in last_go_version_remote.files {
            if f.os == target_os && f.arch == target_arch && f.kind == "archive" {
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
                    Err(error) => println!(" -> Error removing `{}`: {}", local_go_install, error),
                };
                
                println!("Unpacking `{}`", version_file.download_to.as_str());
                match cmd::go_install(go_install_to, version_file.download_to.as_str()) {
                    Ok(output) => println!("{}", output),
                    Err(error) => panic!("Error installing: {}", error),
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
