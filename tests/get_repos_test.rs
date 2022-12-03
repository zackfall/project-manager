use actions::get::repos;

#[tokio::test]
    async fn get_repos_test() {
        let repo = repos::get_repo("zackfall", "project-manager").await;
        match repo {
            Ok(rp) => {
                assert_eq!(rp.name, "project-manager");
                assert_eq!(rp.owner.unwrap().login, "zackfall");
            }
            Err(err) => panic!("{}", err),
        }
    }
