use database::models::config::automod::checks::{CapsLock, Check, Invites, Regex, TextLines};
use utils::ok_or_skip_without_clone;
use crate::ScamLinks;

pub async fn checks_match(check: &Check, message_content: &str, scam_domains: &ScamLinks) -> Result<bool, ()> {
    match check {
        Check::FlaggedScamLink => flagged_scam_link(message_content, scam_domains).await,
        Check::TextLines(config) => Ok(text_lines(config, message_content)),
        Check::CapsLock(config) => Ok(caps_lock(config, message_content)),
        Check::Invites(config) => invites(config, message_content),
        Check::Regex(config) => regex(config, message_content)
    }
}

async fn flagged_scam_link(message_content: &str, scam_domains: &ScamLinks) -> Result<bool, ()> {
    let domains = regex::Regex::new(r"(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z0-9][a-z0-9-]{0,61}[a-z0-9]").map_err(|_| ())?;

    let message_content = message_content.to_lowercase();

    let domains = domains.find_iter(message_content.as_str());
    let domains = domains.map(|domain| domain.as_str().to_string()).collect();
    Ok(scam_domains.contains(domains).await)
}

fn text_lines(config: &TextLines, message_content: &str) -> bool {
    let enters = message_content.lines().count();
    let line_len = if let Some(len) = config.line_len { len as usize } else { 120 };
    let split = message_content.len() / line_len;
    let lines = enters + split;

    min_max_checks(
        config.min.map(usize::from),
        config.max.map(usize::from),
        lines
    )
}

fn caps_lock(config: &CapsLock, message_content: &str) -> bool {
    let uppercase = message_content.chars().filter(|c| c.is_uppercase()).count();
    let uppercase_part = uppercase * 100 / message_content.len();

    min_max_checks(
        config.min.map(usize::from),
        config.max.map(usize::from),
        uppercase_part
    )
}

fn invites(config: &Invites, message_content: &str) -> Result<bool, ()> {
    let invites = regex::Regex::new(r"(?i)(discord.gg|discordapp.com/invite|discord.com/invite)(?:/#)?/([a-zA-Z0-9-]+)").map_err(|_| ())?;

    let message_content = message_content.to_lowercase();

    let invites = invites.find_iter(message_content.as_str());
    let mut contains = false;
    for invite in invites {
        let code = ok_or_skip_without_clone!(invite.as_str().split('/').last(), Some);
        if !config.allowed_invites.contains(&code.to_string()) {
            contains = true;
            break;
        }
    }

    Ok(contains)
}

fn regex(config: &Regex, message_content: &str) -> Result<bool, ()> {
    let regex = regex::Regex::new(&*config.regex).map_err(|_| ())?;
    let is_matching = regex.is_match(message_content);
    Ok((is_matching && config.is_matching) || (!is_matching && !config.is_matching))
}

fn min_max_checks(min: Option<usize>, max: Option<usize>, count: usize) -> bool {
    (if let Some(min) = min {
        count > min
    } else { true }) && (if let Some(max) = max {
        count < max
    } else { true })
}

#[cfg(test)]
mod tests {
    use std::io::repeat;
    use database::models::config::automod::checks::{CapsLock, Invites, Regex, TextLines};
    use crate::modules::automod::checks::{caps_lock, invites, regex, text_lines};
    use crate::ScamLinks;

    #[test]
    fn test_invites() {
        assert_eq!(
            invites(
                &Invites { allowed_invites: vec![] },
                &"discord.gg/discord-developers".to_string()
            ).unwrap(),
            true
        );
        assert_eq!(
            invites(
                &Invites { allowed_invites: vec![] },
                &"discord.com/invite/discord-developers".to_string()
            ).unwrap(),
            true
        );
        assert_eq!(
            invites(
                &Invites { allowed_invites: vec!["discord-developers".to_string()] },
                &"discord.gg/discord-developers".to_string()
            ).unwrap(),
            false
        );
        assert_eq!(
            invites(
                &Invites { allowed_invites: vec![] }, &"".to_string()
            ).unwrap(),
            false
        );
    }

    #[test]
    fn test_caps_lock() {
        assert_eq!(
            caps_lock(&CapsLock { min: None, max: None }, &"ASDH".to_string()),
            true
        );
        assert_eq!(
            caps_lock(&CapsLock { min: Some(2), max: Some(100) }, &"ADAsi".to_string()),
            true
        );
        assert_eq!(
            caps_lock(&CapsLock { min: Some(2), max: Some(10) }, &"A".to_string()),
            false
        );
    }

    #[test]
    fn test_regexp_matching() {
        assert_eq!(
            regex(
                &Regex { is_matching: true, regex: "ok".to_string() },
                &"ok".to_string()
            ).unwrap(),
            true
        );
        assert_eq!(
            regex(
                &Regex { is_matching: false, regex: "ok".to_string() },
                &"ok".to_string()
            ).unwrap(),
            false
        );
        assert_eq!(
            regex(
                &Regex { is_matching: true, regex: "no".to_string() },
                &"ok".to_string()
            ).unwrap(),
            false
        );
    }

    #[test]
    fn test_text_lines() {
        assert_eq!(
            text_lines(
                &TextLines { line_len: Some(80), min: None, max: None },
                &"\n\n\n".to_string()
            ),
            true
        );
        assert_eq!(
            text_lines(
                &TextLines { line_len: Some(80), min: Some(1), max: Some(2) },
                &"\n\n\n".to_string()
            ),
            false
        );
        assert_eq!(
            text_lines(
                &TextLines { line_len: Some(80), min: Some(1), max: None },
                &"".to_string()
            ),
            false
        )
    }

}