use ureq::Agent;

use crate::error::{RunnerError, RunnerResult};
use std::{fs, path::Path};

pub struct Downloader {
    agent: Agent,
    session_token: String,
}

impl Downloader {
    pub fn new(session_token: impl Into<String>) -> Self {
        let agent = Agent::new();
        let session_token = session_token.into();
        Self {
            agent,
            session_token,
        }
    }

    pub fn fetch(&self, year: u16, day: u8, out_dir: impl AsRef<Path>) -> RunnerResult<()> {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let session_token = &self.session_token;
        let response = self
            .agent
            .get(&url)
            .set("Cookie", &format!("session={session_token}"))
            .call()
            .map_err(|err| RunnerError::DownloadError(Box::new(err)))?;

        match response.status() {
            200 => {
                let mut bytes = Vec::new();
                response
                    .into_reader()
                    .read_to_end(&mut bytes)
                    .map_err(|err| RunnerError::IoError {
                        path: Path::new(&url).to_path_buf(),
                        err,
                    })?;
                self.save_to_file(&bytes, day, out_dir)
            }
            _status => todo!("Received unexpected status code"),
        }
    }

    fn save_to_file(&self, bytes: &[u8], day: u8, out_dir: impl AsRef<Path>) -> RunnerResult<()> {
        let out_dir = out_dir.as_ref();
        fs::create_dir_all(out_dir).map_err(|err| RunnerError::IoError {
            path: out_dir.to_path_buf(),
            err,
        })?;

        let out_file = out_dir.join(format!("day{day:02}.txt"));
        fs::write(out_file, bytes).map_err(|err| RunnerError::IoError {
            path: out_dir.to_path_buf(),
            err,
        })
    }
}
