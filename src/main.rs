use dash::{dashboard::domain::UserDashboard, shared::domain::UserId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let owner = UserId::try_from("local-user")?;
    let _dashboard = UserDashboard::new(owner);
    Ok(())
}
