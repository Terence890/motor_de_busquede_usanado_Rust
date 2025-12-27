use clap::Parser;
use fast_search_engine_rust::api::Api;
use fast_search_engine_rust::query::normalize_query;

#[derive(Parser, Debug)]
#[command(author, version, about = "Minimal fast search engine demo")]
struct Args {
	/// Query to run (if omitted, runs an interactive demo)
	query: Option<String>,
}

fn main() {
	let args = Args::parse();
	let mut api = Api::new();

	// sample documents
	let docs = vec![
		"The quick brown fox",
		"jumps over the lazy dog",
		"Rust is blazingly fast",
		"Search engines rank results",
	];

	for (i, d) in docs.iter().enumerate() {
		api.index_doc(i, d);
	}

	if let Some(q) = args.query {
		let nq = normalize_query(&q);
		let results = api.search(&nq);
		for d in results {
			println!("[{}] {}", d.id, d.text);
		}
	} else {
		println!("Indexed {} documents.", docs.len());
		println!("Run with: cargo run -- 'search terms'");
	}
}
