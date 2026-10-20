use dash::domain::{dashboard::UserDashboard, user::UserId};

fn main() {
    let owner = UserId::new("local-user").expect("user ID must not be empty");
    let _dashboard = UserDashboard::new(owner);
}
