use crate::helpers::{gitea::gitea_address, reqwest_utils::ReqwestUtils};

use super::{GiteaContext, GiteaUser};

impl GiteaContext {
    pub fn repo_clone_url(&self) -> String {
        format!(
            // if you need ssh instead of http: "ssh://git@localhost:2222/{}/{}.git",
            "http://{}:{}@{}/{}/{}.git",
            self.user.username(),
            self.user.password(),
            gitea_address(),
            self.user.username(),
            self.repo
        )
    }

    pub async fn repo_exists(&self, repo_name: &str) -> bool {
        let repo = self.get_repo(repo_name).await;
        repo == repo_name
    }

    fn pull_url(&self, pr_number: u64) -> String {
        format!("{}/pulls/{}", self.repo_url(), pr_number)
    }

    fn repo_url(&self) -> String {
        self.specific_repo_url(&self.repo)
    }

    fn specific_repo_url(&self, repo_name: &str) -> String {
        super::gitea_endpoint(&format!("repos/{}/{}", self.user.username, repo_name))
    }

    /// Get the repository and return its name.
    async fn get_repo(&self, repo_name: &str) -> String {
        let repo: Repository = self
            .client
            .get(&self.specific_repo_url(repo_name))
            .basic_auth(&self.user.username, Some(&self.user.password))
            .send()
            .await
            .unwrap()
            .ok_if_2xx()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        repo.name
    }

    pub async fn changed_files_in_pr(&self, pr_number: u64) -> Vec<ChangedFile> {
        let pr_url = format!("{}/files", self.pull_url(pr_number));
        self.client
            .get(&pr_url)
            .basic_auth(&self.user.username, Some(&self.user.password))
            .send()
            .await
            .unwrap()
            .ok_if_2xx()
            .await
            .unwrap()
            .json::<Vec<ChangedFile>>()
            .await
            .unwrap()
    }

    pub async fn get_file_content(&self, branch: &str, file_path: &str) -> String {
        use base64::Engine as _;
        let request_path = format!("{}/contents/{}", self.repo_url(), file_path);
        let response = self
            .client
            .get(&request_path)
            .basic_auth(&self.user.username, Some(&self.user.password))
            .query(&[("ref", branch)])
            .send()
            .await
            .unwrap()
            .ok_if_2xx()
            .await
            .unwrap()
            .json::<Contents>()
            .await
            .unwrap();
        let content = base64::engine::general_purpose::STANDARD
            .decode(response.content.as_bytes())
            .unwrap();
        String::from_utf8(content).unwrap()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Contents {
    pub content: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChangedFile {
    pub filename: String,
}

impl GiteaUser {
    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(serde::Deserialize)]
struct Repository {
    name: String,
}
