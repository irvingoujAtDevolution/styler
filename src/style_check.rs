use std::path::Path;

pub fn check_files(path: impl AsRef<Path>) -> anyhow::Result<()> {
    assert!(path.as_ref().is_file());
    assert!(path.as_ref().extension().map_or(false, |e| e == "rs"));
    let content = std::fs::read_to_string(path)?;
    check_for_logging(&content)?;

    Ok(())
}

fn check_for_logging(content: &str) -> anyhow::Result<()> {
    // Iterate through each line in the content
    for (line_num, line) in content.lines().enumerate() {
        // Extract the log macro calls (e.g., `info!`, `error!`, `debug!`, etc.)
        if let Some(log_call) = extract_log_call(line) {
            // Validate the log message against style rules
            validate_log_message(log_call, line_num + 1)?;
        }
    }
    Ok(())
}

fn extract_log_call(line: &str) -> Option<&str> {
    // Look for logging macros
    let log_macros = ["info!", "error!", "debug!", "trace!", "warn!"];
    log_macros.iter().find_map(|&macro_name| {
        if let Some(start) = line.find(macro_name) {
            line[start..].splitn(2, '(').nth(1).map(|s| s.trim())
        } else {
            None
        }
    })
}

fn validate_log_message(log_call: &str, line_num: usize) -> anyhow::Result<()> {
    // Extract the log message (first argument in the macro)
    if let Some(message) = log_call.splitn(2, ',').next() {
        let trimmed_message = message.trim_matches(|c| c == '"' || c == '\'' || c == '`');

        // Check if the message starts with a capital letter
        if !trimmed_message
            .chars()
            .next()
            .map_or(false, |c| c.is_uppercase())
        {
            anyhow::bail!(
                "Line {}: Log message does not start with a capital letter: {}",
                line_num,
                trimmed_message
            );
        }

        // Check if the message ends with a period
        if trimmed_message.ends_with('.') {
            anyhow::bail!(
                "Line {}: Log message ends with a period: {}",
                line_num,
                trimmed_message
            );
        }

        // Check for inline interpolation (e.g., `:`, but no `%` in structured fields)
        if trimmed_message.contains(':') && !log_call.contains('%') {
            anyhow::bail!(
                "Line {}: Consider using structured fields instead of inline interpolation: {}",
                line_num,
                trimmed_message
            );
        }
    }

    Ok(())
}
