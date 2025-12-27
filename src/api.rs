use crate::indexer::InvertedIndex;

pub struct Api {
	idx: InvertedIndex,
}

impl Api {
	pub fn new() -> Self {
		Self { idx: InvertedIndex::new() }
	}

	pub fn index_doc(&mut self, id: usize, text: &str) {
		self.idx.add_doc(id, text);
	}

	pub fn search(&self, q: &str) -> Vec<&crate::indexer::Doc> {
		self.idx.search(q)
	}
}
