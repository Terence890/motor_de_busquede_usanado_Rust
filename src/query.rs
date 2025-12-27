pub fn normalize_query(q: &str) -> String {
	q.split_whitespace()
		.map(|s| s.to_lowercase())
		.collect::<Vec<_>>()
		.join(" ")
}
