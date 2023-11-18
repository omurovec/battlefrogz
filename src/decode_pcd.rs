pub fn decode_pcd(json_str: &str) -> Option<(u32, u32, u32, u32)> {
    let key_intelligence = "\\\"intelligence\\\":";
    let key_beauty = "\\\"beauty\\\":";
    let key_speed = "\\\"speed\\\":";
    let key_jump = "\\\"jump\\\":";

    let intelligence = extract_value(json_str, key_intelligence)?;
    let beauty = extract_value(json_str, key_beauty)?;
    let speed = extract_value(json_str, key_speed)?;
    let jump = extract_value(json_str, key_jump)?;

    Some((intelligence, beauty, speed, jump))
}

fn extract_value(json_str: &str, key: &str) -> Option<u32> {
    json_str.find(key).and_then(|start| {
        let value_start = start + key.len();
        json_str[value_start..]
            .split(',')
            .next()?
            .split('}')
            .next()?
            .trim()
            .parse()
            .ok()
    })
}
