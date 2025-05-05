use dioxus::prelude::*;
use directories::ProjectDirs;
use itertools::Itertools;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
  env,
  path::{Path, PathBuf},
  str::FromStr,
};
use thiserror::Error;
use tokio::{fs, io};

use crate::{
  board::HyphaBoard, container::HyphaContainer, dep::HyphaDep,
  r#ref::HyphaFileIssueRef,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HyphaFile {
  #[serde(default)]
  pub boards: Vec<HyphaBoard>,
  #[serde(default)]
  pub deps: Vec<HyphaDep<HyphaFileIssueRef>>,
  #[serde(skip)]
  pub path: PathBuf,
}

impl HyphaContainer for HyphaFile {
  type Item = HyphaBoard;
  type Ref = HyphaFileIssueRef;

  fn items(&self) -> &Vec<Self::Item> {
    &self.boards
  }

  fn items_mut(&mut self) -> &mut Vec<Self::Item> {
    &mut self.boards
  }
}

impl Default for HyphaFile {
  fn default() -> Self {
    let boards = vec![HyphaBoard::default()];
    if let Ok(path) = HyphaFile::path() {
      HyphaFile {
        path,
        boards,
        deps: vec![],
      }
    } else {
      HyphaFile {
        boards,
        path: PathBuf::new(),
        deps: vec![],
      }
    }
  }
}

#[component]
pub fn Summary(file: HyphaFile) -> Element {
  let path = file.path.to_str().unwrap_or_default();
  rsx! {
    span { {path} }
  }
}

#[component]
pub fn Details(file: HyphaFile) -> Element {
  let path = file.path.to_str().unwrap_or_default();
  let boards = file
    .boards
    .iter()
    .map(|board| board.title.as_str())
    .collect::<Vec<_>>()
    .join(", ");

  rsx! {
    p { {path} }
    p { {boards} }
  }
}

#[component]
pub fn Edit(file: HyphaFile, on_change: EventHandler<HyphaFile>) -> Element {
  let path = file.path.to_str().unwrap_or("");
  rsx! {
    p {
      input {
        value: path,
        oninput: {
          let value = file.clone();
          move |e: Event<FormData>| {
            let path = PathBuf::from_str(e.value().as_str()).unwrap_or(PathBuf::new());
            let mut value = value.clone();
            value.path = path;
            on_change(value);
          }
        }
      }
    }
  }
}

impl HyphaFile {
  #[allow(dead_code, reason = "I like these")]
  pub fn load(path: &Path) -> Result<HyphaFile, FileError> {
    info!("Attempting to load config from: {}", path.display());
    match std::fs::read_to_string(path) {
      Ok(content) => {
        let mut file: HyphaFile = toml::from_str(&content)?;
        file.path = path.to_path_buf();
        file.uniq();
        info!("Hypha file loaded successfully.");
        Ok(file)
      }
      Err(e) if e.kind() == io::ErrorKind::NotFound => {
        warn!(
          "Hypha file not found at {}, creating default.",
          path.display()
        );
        Ok(HyphaFile::default())
      }
      Err(e) => {
        error!("Failed to read hypha file: {e}");
        Err(FileError::Io(e))
      }
    }
  }

  pub async fn load_async(path: &Path) -> Result<HyphaFile, FileError> {
    info!("Attempting to load config from: {}", path.display());
    match fs::read_to_string(path).await {
      Ok(content) => {
        let mut file: HyphaFile = toml::from_str(&content)?;
        file.path = path.to_path_buf();
        file.uniq();
        info!("Hypha file loaded successfully.");
        Ok(file)
      }
      Err(e) if e.kind() == io::ErrorKind::NotFound => {
        warn!(
          "Hypha file not found at {}, creating default.",
          path.display()
        );
        Ok(HyphaFile::default())
      }
      Err(e) => {
        error!("Failed to read hypha file: {e}");
        Err(FileError::Io(e))
      }
    }
  }

  pub fn path() -> Result<PathBuf, FileError> {
    if let Ok(path_str) = env::var("HYPHA_FILE") {
      let expanded_path = shellexpand::full(&path_str)
        .map_err(|e| FileError::Expansion(path_str.clone(), e.to_string()))?;
      info!("Using hypha file path from HYPHA_FILE env var: {expanded_path}");
      return Ok(PathBuf::from(expanded_path.to_string()));
    }

    if let Some(proj_dirs) = ProjectDirs::from("com", "HyphaApp", "Hypha") {
      let data_dir = proj_dirs.data_local_dir();
      let default_path = data_dir.join("hypha.toml");
      info!("Using default hypha file path: {}", default_path.display());
      Ok(default_path)
    } else {
      error!(
        "Could not determine project directory for default hypha file path."
      );
      Err(FileError::Directory)
    }
  }

  #[allow(dead_code, reason = "I like these")]
  pub fn reload(&mut self) -> Result<(), FileError> {
    let loaded = Self::load(&self.path)?;
    *self = loaded;
    Ok(())
  }

  #[allow(dead_code, reason = "I like these")]
  pub async fn reload_async(&mut self) -> Result<(), FileError> {
    let loaded = Self::load_async(&self.path).await?;
    *self = loaded;
    Ok(())
  }

  pub fn save(&self) -> Result<(), FileError> {
    let path = self.path.as_path();

    info!("Attempting to save hypha file to: {}", path.display());

    if let Some(parent_dir) = path.parent() {
      std::fs::create_dir_all(parent_dir)?;
      info!("Ensured hypha directory exists: {}", parent_dir.display());
    }

    let toml_string = toml::to_string_pretty(self)?;
    std::fs::write(path, toml_string)?;
    info!("Hypha file saved successfully to: {}", path.display());
    Ok(())
  }

  pub async fn save_async(&self) -> Result<(), FileError> {
    let path = self.path.as_path();

    info!("Attempting to save hypha file to: {}", path.display());

    if let Some(parent_dir) = path.parent() {
      fs::create_dir_all(parent_dir).await?;
      info!("Ensured hypha directory exists: {}", parent_dir.display());
    }

    let toml_string = toml::to_string_pretty(self)?;
    fs::write(path, toml_string).await?;
    info!("Hypha file saved successfully to: {}", path.display());
    Ok(())
  }

  fn uniq(&mut self) {
    self.boards = self
      .boards
      .iter()
      .cloned()
      .unique_by(|board| board.title.clone())
      .collect();

    for board in self.boards.iter_mut() {
      board.lists = board
        .lists
        .iter()
        .cloned()
        .unique_by(|list| list.title.clone())
        .collect();
    }

    for board in self.boards.iter_mut() {
      for list in board.lists.iter_mut() {
        list.issues = list
          .issues
          .iter()
          .cloned()
          .unique_by(|issue| issue.title.clone())
          .collect();
      }
    }
  }
}

#[derive(Error, Debug)]
pub enum FileError {
  #[error("IO error: {0}")]
  Io(#[from] io::Error),

  #[error("TOML serialization error: {0}")]
  TomlSer(#[from] toml::ser::Error),

  #[error("TOML deserialization error: {0}")]
  TomlDe(#[from] toml::de::Error),

  #[error("Environment variable error: {0}")]
  EnvVar(#[from] env::VarError),

  #[error("Could not determine project directory for config path")]
  Directory,

  #[error("Failed to expand path '{0}': {1}")]
  Expansion(String, String),
}
