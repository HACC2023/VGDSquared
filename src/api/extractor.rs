use poem::Request;

pub async fn auth_extractor(req: &Request) -> poem::Result<Vec<String>> {
    let map = req.headers();
    let header = if let Some(header) = map.get("Authorization") {
        header
    } else {
        return Ok(Vec::new());
    };

    

    todo!()
}
