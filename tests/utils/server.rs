pub fn generate_create_body(
    name: &str,
    description: Option<&str>,
    photo: Option<&str>,
    cover_photo: Option<&str>,
) -> String {
    format!(
        r#"
    {{
        "name": "{}",
        "description": {},
        "photo": {},
        "cover_photo": {}
    }}
    "#,
        name,
        description.map_or("null", |d| format!(r#""{}""#, d).leak()),
        photo.map_or("null", |p| format!(r#""{}""#, p).leak()),
        cover_photo.map_or("null", |c| format!(r#""{}""#, c).leak()),
    )
}
