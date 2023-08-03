use std::process::Command;
use std::path;
use std::str;
use std::{error::Error, fmt};
use std::fs;
use std::cmp::PartialEq;

use regex::Regex;
use os_info;

#[derive(Debug)]
pub struct GoVersion {
    pub version: String,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    version_float: f64, 
}

impl GoVersion {
    pub fn new(version_str: String) -> Self {
        //Remove the "go" word, if any
        let go_version = version_str.replace("go", "");
        let scheme: Vec<&str> = go_version.split('.').collect();
        let mut major: i32 = 0;
        let mut minor: i32 = 0;
        let mut patch: i32 = 0;
        let mut version_float: f64 = 0.0;
        if scheme.len() == 3 {
            major = conv_to::<i32>(scheme[0]).unwrap_or_default();
            minor = conv_to::<i32>(scheme[1]).unwrap_or_default();
            patch = conv_to::<i32>(scheme[2]).unwrap_or_default();
            let v_f64 = scheme[0].to_owned() + scheme[1] + "." + scheme[2];
            version_float = conv_to::<f64>(v_f64.as_str()).unwrap_or_default();
        }

        Self {
            version: go_version,
            major,
            minor,
            patch,
            version_float,
        }
    }

    pub fn gt(&self, other: &GoVersion) -> bool {
        if self.version_float > other.version_float {
            return true;
        }
        false
    }
}

impl PartialEq for GoVersion {
    fn eq(&self, other: &GoVersion) -> bool {
        self.version_float == other.version_float
    }
}

#[derive(Debug)]
pub struct GoError {
    pub message: String,
}

impl Error for GoError {}

impl fmt::Display for GoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct ShellCommand {
    cmd: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn new(cmd: &str, args: &[&str]) -> Self {
        let mut vec_args: Vec<String> = vec![];
        for arg in args {
            vec_args.push(String::from(arg.to_owned()));
        }
        Self {
            cmd: String::from(cmd),
            args: vec_args,
        }
    }

    pub fn exec(&self) -> Result<String, GoError> {
        let cmd = Command::new(self.cmd.as_str())
            .args(self.args.as_slice())
            .output()
            .unwrap_or_else(|_| panic!("Failed to run `{}`: Is it in your PATH/installed?", self.cmd.as_str()));
        if let Ok(output) = str::from_utf8(cmd.stdout.as_slice()) {
            return Ok(String::from(output));
        }
        Err(
            GoError {
                message: String::from("Go cmd ran, but failed to get the output!!!")
            }
        )    
    }
}

pub struct GoCommand {
    bin_path: path::PathBuf,
}

impl GoCommand {
    pub fn new(bin_path: path::PathBuf) -> Self {
        Self {
            bin_path
        }
    }

    pub fn version(&self) -> Result<GoVersion, GoError> {
        let go_bin: path::PathBuf = [self.bin_path.to_str().unwrap_or(""), "go"].iter().collect();
        let output = ShellCommand::new(go_bin.to_str().unwrap_or("go"), &["version"]).exec()?;
        Ok(GoVersion::new(parse_version(output.as_str())))    
    }

    pub fn root(&self) -> Result<String, GoError> {
        let go_bin: path::PathBuf = [self.bin_path.to_str().unwrap_or(""), "go"].iter().collect();
        let cmd = ShellCommand::new(go_bin.to_str().unwrap_or("go"), &["env", "GOROOT"]);
        match cmd.exec() {
            Ok(output) => Ok(str::replace(output.as_str(), "\n", "")),
            Err(error) => Err(error),
        }    
    }
}

fn parse_version(version_str: &str) -> String {
    let re = Regex::new(r"^.*(go\d+\.\d+\.\d+).*").unwrap();
    match re.captures(version_str) {
        Some(cap) => String::from(cap.get(1).unwrap().as_str()),
        None => String::from(""),
    }
}

pub fn remove_dir(dir: &str) -> std::io::Result<()> {
    fs::remove_dir_all(dir)
}

pub fn go_install(install_to: &str, install_from: &str) -> Result<String, GoError> {
    let cmd = ShellCommand::new("tar", &["-C", install_to, "-xzf", install_from]);
    cmd.exec()
}

fn conv_to<T: std::str::FromStr>(s: &str) -> Option<T> {
    if let Ok(result) = s.parse::<T>() {
        return Some(result);
    }
    None
}

pub fn os_type() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        Some("linux".to_string())
    }
    #[cfg(any(target_vendor = "apple", target_os = "windows"))]
    {
        let typ = unsafe { ffi::CStr::from_ptr(get_os_type() as *const c_char).to_bytes() };
        Some(String::from_utf8_lossy(typ).into_owned())
    }
    #[cfg(target_os = "solaris")]
    {
        Some("solaris".to_string())
    }
    #[cfg(target_os = "illumos")]
    {
        Some("illumos".to_string())
    }
    #[cfg(target_os = "freebsd")]
    {
        Some("freebsd".to_string())
    }
    #[cfg(target_os = "openbsd")]
    {
        Some("openbsd".to_string())
    }
    #[cfg(target_os = "netbsd")]
    {
        Some("netbsd".to_string())
    }
    #[cfg(not(any(target_os = "linux", target_vendor = "apple", target_os = "windows", target_os = "solaris", target_os = "illumos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd")))]
    {
        None
    }
}

pub fn os_arch() -> String {
    let info = os_info::get();
    let arch = info.architecture().unwrap_or("Unknown");
    match arch {
        "x86_64" => "amd64".to_string(),
        "x86"    => "386".to_string(),
        _        => arch.to_string(),
    }
}