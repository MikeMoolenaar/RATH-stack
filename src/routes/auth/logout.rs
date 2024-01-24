use crate::models::user::User;
use axum_htmx::HxLocation;
use tower_sessions::Session;

pub async fn logout(session: Session) -> (HxLocation, &'static str) {
    session.remove::<User>("user").unwrap();
    return (HxLocation::from_str("/login").unwrap(), "");
}
