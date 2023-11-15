use poem::Request;

/// I want this to use `Permission` but not sure on how to switch it out while making the compiler happy
pub async fn auth_extractor(req: &Request) -> poem::Result<Vec<String>> {
    let map = req.headers();
    let header = if let Some(header) = map.get("Authorization") {
        header
    } else {
        return Ok(Vec::new());
    };

    Ok(vec!["moderator".to_string()])
}

pub enum Permission {
    Trusted,
    Admin,
}