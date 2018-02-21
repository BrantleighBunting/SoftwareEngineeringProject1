/* Parse Tree */
pub struct Node <T> {
	child: Vec<T>,
	entry: T
}

impl <T> Node <T> {
	pub fn new(entry: T) -> Node <T> {
		return Node {
			child: Vec::new(),
			entry: entry
		}
	}
}