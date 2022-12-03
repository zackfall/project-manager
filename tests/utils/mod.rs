pub struct Data {
    pub owner: String,
    pub repo: String,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            owner: "zackfall".to_owned(),
            repo: "project-manager".to_owned(),
        }
    }
}
