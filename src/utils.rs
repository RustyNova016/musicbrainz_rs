use regex::Regex;

/// Checks is a string is an UUID, the format for musicbrainz mbids
pub fn is_string_uuid(string: &str) -> bool {
    let regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let result = regex.captures(string);

    result.is_some()
}

/// Extract the mbids mbid from known Musicbrainz/Listenbrainz URLs. It doesn't return the type of the mbid
pub fn get_mbid_from_url(string: &str) -> Option<String> {
    let regex = Regex::new(r"(area|artist|event|instrument|label|place|recording|release|release-group|album|series|work|url)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let caps = regex.captures(string)?;

    Some(caps.get(2)?.as_str().to_string())
}

pub fn parse_mbid(input: &str) -> Option<String> {
    if is_string_uuid(input) {
        return Some(input.to_string());
    }

    get_mbid_from_url(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_string_uuid() {
        assert!(is_string_uuid("fb91e2b9-4a35-4ebc-8cc4-0dcf6443ad81"));
        assert!(!is_string_uuid("not-a-uuid"));
        assert!(!is_string_uuid("fb91e2b94a354ebc8cc40dcf6443ad81"));
        assert!(!is_string_uuid("fb91e2b9-4a35-4ebc-8cc4-0dcf6443ad810"));
    }

    #[test]
    fn test_get_mbid_from_url() {
        assert_eq!(
            get_mbid_from_url(
                "https://musicbrainz.org/artist/550e8400-e29b-41d4-a716-446655440000"
            ),
            Some("550e8400-e29b-41d4-a716-446655440000".to_string())
        );
        assert_eq!(
            get_mbid_from_url(
                "https://beta.musicbrainz.org/recording/550e8400-e29b-41d4-a716-446655440000"
            ),
            Some("550e8400-e29b-41d4-a716-446655440000".to_string())
        );
        assert_eq!(
            get_mbid_from_url(
                "https://test.musicbrainz.org/label/550e8400-e29b-41d4-a716-446655440000"
            ),
            Some("550e8400-e29b-41d4-a716-446655440000".to_string())
        );
        assert_eq!(
            get_mbid_from_url(
                "https://listenbrainz.org/album/77d9c0f5-77ee-4296-847e-d03ffa4b0b7f/"
            ),
            Some("77d9c0f5-77ee-4296-847e-d03ffa4b0b7f".to_string())
        );
        assert_eq!(
            get_mbid_from_url("https://musicbrainz.org/artist/not-a-uuid"),
            None
        );
        assert_eq!(
            get_mbid_from_url(
                "https://musicbrainz.org/unknown/550e8400-e29b-41d4-a716-446655440000"
            ),
            None
        );
    }
}
