use poem::{Request, web::RemoteAddr};
use tracing::info;

/// I want this to use `Permission` but not sure on how to switch it out while making the compiler happy
pub async fn auth_extractor(req: &Request) -> poem::Result<Vec<String>> {
    info!("{:#?}", req);
    // let map = req.headers();
    // let header = if let Some(header) = map.get("Authorization") {
    //     header
    // } else {
    //     return Ok(Vec::new());
    // };

    // Ok(vec!["moderator".to_string()])
    Ok(Vec::new())
}

pub enum Permission {
    Trusted,
    Admin,
}