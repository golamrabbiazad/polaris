use std::path::PathBuf;

use crate::app::{config, index, ndb, playlist, scanner};
use crate::test::*;

use super::config::AuthSecret;

pub struct Context {
	pub index_manager: index::Manager,
	pub scanner: scanner::Scanner,
	pub config_manager: config::Manager,
	pub playlist_manager: playlist::Manager,
}

pub struct ContextBuilder {
	config: config::Config,
	pub test_directory: PathBuf,
}

impl ContextBuilder {
	pub fn new(test_name: String) -> Self {
		Self {
			test_directory: prepare_test_directory(test_name),
			config: config::Config::default(),
		}
	}

	pub fn user(mut self, name: &str, password: &str, is_admin: bool) -> Self {
		self.config.users.push(config::User {
			name: name.to_owned(),
			initial_password: Some(password.to_owned()),
			admin: Some(is_admin),
			..Default::default()
		});
		self
	}

	pub fn mount(mut self, name: &str, source: &str) -> Self {
		self.config.mount_dirs.push(config::MountDir {
			name: name.to_owned(),
			source: source.to_owned(),
		});
		self
	}
	pub async fn build(self) -> Context {
		let config_path = self.test_directory.join("polaris.toml");

		let auth_secret = AuthSecret::default();
		let config_manager = config::Manager::new(&config_path).await.unwrap();
		let ndb_manager = ndb::Manager::new(&self.test_directory).unwrap();
		let index_manager = index::Manager::new(&self.test_directory).await.unwrap();
		let scanner = scanner::Scanner::new(index_manager.clone(), config_manager.clone())
			.await
			.unwrap();
		let playlist_manager = playlist::Manager::new(ndb_manager.clone());

		config_manager.apply(self.config).await.unwrap();

		Context {
			index_manager,
			scanner,
			config_manager,
			playlist_manager,
		}
	}
}
