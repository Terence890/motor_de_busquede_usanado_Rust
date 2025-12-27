use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

pub type DocId = usize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Doc {
	pub id: DocId,
	pub text: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InvertedIndex {
	map: HashMap<String, Vec<DocId>>,
	docs: HashMap<DocId, Doc>,
}

impl InvertedIndex {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
			docs: HashMap::new(),
		}
	}

	pub fn add_doc(&mut self, id: DocId, text: &str) {
		let doc = Doc { id, text: text.to_string() };
		let tokens: HashSet<_> = text
			.split_whitespace()
			.map(|s| s.to_lowercase())
			.collect();

		for token in tokens {
			self.map.entry(token).or_default().push(id);
		}
		self.docs.insert(id, doc);
	}

	pub fn search(&self, query: &str) -> Vec<&Doc> {
		let tokens: Vec<_> = query
			.split_whitespace()
			.map(|s| s.to_lowercase())
			.collect();

		if tokens.is_empty() {
			return vec![];
		}

		let mut sets: Vec<HashSet<DocId>> = Vec::new();
		for t in tokens {
			if let Some(list) = self.map.get(&t) {
				sets.push(list.iter().cloned().collect());
			} else {
				return vec![];
			}
		}

		let mut iter = sets.into_iter();
		let first = iter.next().unwrap();
		let intersection = iter.fold(first, |acc, s| {
			acc.intersection(&s).cloned().collect()
		});

		let mut results: Vec<&Doc> = intersection
			.into_iter()
			.filter_map(|id| self.docs.get(&id))
			.collect();

		results.sort_by_key(|d| d.id);
		results
	}

	pub fn docs_count(&self) -> usize {
		self.docs.len()
	}
}

