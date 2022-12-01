use dotenv_codegen::dotenv;
use octocrab::Octocrab;

pub mod get_actions;

pub fn build_octo() -> Octocrab {
    let token = dotenv!("GH_TOKEN");
    let octo = octocrab::OctocrabBuilder::new()
        .personal_token(token.to_owned())
        .build();
    match octo {
        Ok(octo) => octo,
        Err(err) => panic!("{}", err),
    }
}
