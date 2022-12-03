mod utils;
use actions::get::users;
use utils::Data;

#[tokio::test]
async fn current_user_test() {
    let data = Data::default();
    let user = users::get_current_user().await;
    match user {
        Ok(usr) => assert_eq!(usr.login, data.owner),
        Err(_) => panic!("Nicknames don't match"),
    }
}
