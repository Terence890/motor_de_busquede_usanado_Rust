use fast_search_engine_rust::InvertedIndex;

#[test]
fn basic_index_search() {
	let mut idx = InvertedIndex::new();
	idx.add_doc(1, "Hello world");
	idx.add_doc(2, "Hello rust");

	let res = idx.search("hello");
	assert_eq!(res.len(), 2);

	let res2 = idx.search("rust");
	assert_eq!(res2.len(), 1);
	assert_eq!(res2[0].id, 2);
}
