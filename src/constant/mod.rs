use chrono::FixedOffset;

pub static LOCAL_TIMEZONE: FixedOffset = match FixedOffset::east_opt(8 * 3600) {
    Some(offset) => offset,
    None => panic!("Failed to parse timezone"),
};
