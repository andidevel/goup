pub mod cmd;
pub mod req;

pub fn get_new_version() -> Result<req::VersionItem, Box<dyn std::error::Error>> {
    println!("Checking last Golang version...");
    let go_response = req::get()?;
    // Usually, the first item is the latest version available
    if !go_response.is_empty() {
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
