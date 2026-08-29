// bookmarks/mod.rs
use crate::error::{HyraxError, HyraxResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn bookmarks_path() -> HyraxResult<PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("Hyraxql").join("bookmarks.toml"))
        .ok_or_else(|| HyraxError::EngineInit("Couldn't find the config folder".to_owned()))
}

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub id: u64,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Bookmarks {
    #[serde(default)]
    next_id: u64,
    #[serde(default)]
    bookmarks: Vec<Bookmark>,
}

impl Bookmarks {
    /// Load bookmarks from disk, or return an empty set if the file doesn't exist / is empty.
    pub fn new() -> HyraxResult<Self> {
        let path = bookmarks_path()?;
        if !path.exists() {
            return Ok(Self {
                next_id: 0,
                bookmarks: Vec::new(),
            });
        }
        let raw = fs::read_to_string(path)?;
        if raw.trim().is_empty() {
            return Ok(Self {
                next_id: 0,
                bookmarks: Vec::new(),
            });
        }
        Ok(toml::from_str(&raw)?)
    }

    /// Write the current in-memory state to disk (creating the config dir if needed).
    fn save(&self) -> HyraxResult<()> {
        let path = bookmarks_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let raw = toml::to_string_pretty(self)?;
        fs::write(path, raw)?;
        Ok(())
    }

    /// Add a new bookmark, assigning it the next incremental id. Persists to disk.
    pub fn add(&mut self, value: String) -> HyraxResult<u64> {
        let id = self.next_id;
        self.next_id += 1;
        self.bookmarks.push(Bookmark { id, value });
        self.save()?;
        Ok(id)
    }

    /// Remove a bookmark by id, if present. Persists to disk.
    pub fn remove(&mut self, id: u64) -> HyraxResult<()> {
        self.bookmarks.retain(|b| b.id != id);
        self.save()
    }

    /// Return all bookmarks currently in memory.
    pub fn list(&self) -> &[Bookmark] {
        &self.bookmarks
    }
}
