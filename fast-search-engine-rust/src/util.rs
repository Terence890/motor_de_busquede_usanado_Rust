pub fn tokenize(s: &str) -> Vec<String> {
	s.split_whitespace().map(|t| t.to_lowercase()).collect()
}
