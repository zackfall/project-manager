extern crate dotenv;

use octocrab::Octocrab;
use dotenv::dotenv;

pub mod get_actions;

pub fn build_octo() -> Octocrab {
    dotenv().ok();
    let token = std::env::var("TOKEN_GITHUB").ok().unwrap();
    let octo = octocrab::OctocrabBuilder::new()
        .personal_token(token)
        .build();
    match octo {
        Ok(octo) => octo,
        Err(err) => panic!("{}", err),
    }
}
