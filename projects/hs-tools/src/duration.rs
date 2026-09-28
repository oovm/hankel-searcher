use std::time::Duration;

/// Parse a wall-clock budget such as `30m`, `1h`, `90s`.
pub fn parse_duration(input: &str) -> Result<Duration, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("duration must not be empty".into());
    }
    let (number, unit) = trimmed
        .chars()
        .position(|c| !c.is_ascii_digit())
        .map(|idx| trimmed.split_at(idx))
        .ok_or_else(|| format!("invalid duration `{input}`"))?;
    if number.is_empty() {
        return Err(format!("invalid duration `{input}`"));
    }
    let value = number.parse::<u64>().map_err(|_| format!("invalid duration `{input}`"))?;
    let seconds = match unit {
        "s" | "sec" | "secs" => value,
        "m" | "min" | "mins" => value * 60,
        "h" | "hr" | "hrs" => value * 3600,
        _ => return Err(format!("unsupported duration unit in `{input}`")),
    };
    Ok(Duration::from_secs(seconds))
}
