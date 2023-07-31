use self::user_stats::get_mob_killed_counts_for_user;

pub mod core;
pub mod plugin;
pub mod user_stats;

pub fn print_mob_kills(user_id: isize) -> String {
    get_mob_killed_counts_for_user(user_id)
        .into_iter()
        .map(|(mobtype, n)| format!("{mobtype}: {n}"))
        .collect::<Vec<String>>()
        .join("\n")
}
