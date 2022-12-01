use dotenv::dotenv;
use octocrab::Octocrab;

pub mod get_actions;

pub fn build_octo() -> Octocrab {
    dotenv().ok();
    let token = std::env::var("GH_TOKEN");
    let octo = octocrab::OctocrabBuilder::new()
        .personal_token(token.ok().unwrap())
        .build();
    match octo {
        Ok(octo) => octo,
        Err(err) => panic!("{}", err),
    }
}
