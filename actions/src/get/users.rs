use super::super::build_octo;
use octocrab::{models::User, Result};

pub async fn get_current_user() -> Result<User> {
    let current_user = build_octo().current().user().await?;
    Ok(current_user)
}
