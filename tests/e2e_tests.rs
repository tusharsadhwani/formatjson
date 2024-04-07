use std::error::Error;

use formatjson::format_json;

#[test]
fn test_json_example() -> Result<(), Box<dyn Error>> {
    let input_json = include_str!("./json_data/simple.json");
    let expected_json = include_str!("./json_data/simple.formatted.json");
    assert_eq!(format_json(input_json)?, expected_json);
    Ok(())
}

#[test]
fn test_json_userdata() -> Result<(), Box<dyn Error>> {
    let input_json = include_str!("./json_data/userdata.json");
    let expected_json = include_str!("./json_data/userdata.formatted.json");
    assert_eq!(format_json(input_json)?, expected_json);
    Ok(())
}
