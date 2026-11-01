use dash::{dashboard::domain::UserDashboard, shared::domain::UserId};

/// Result returned by the command-line entry point.
type MainResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let owner = UserId::try_from("local-user")?;
    let _dashboard = UserDashboard::new(owner);
    Ok(())
}
