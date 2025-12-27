use crate::indexer::InvertedIndex;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn save_index<P: AsRef<Path>>(idx: &InvertedIndex, path: P) -> Result<()> {
	let s = serde_json::to_string(idx)?;
	fs::write(path, s)?;
	Ok(())
}

pub fn load_index<P: AsRef<Path>>(path: P) -> Result<InvertedIndex> {
	let s = fs::read_to_string(path)?;
	let idx: InvertedIndex = serde_json::from_str(&s)?;
	Ok(idx)
}
