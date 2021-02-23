use std::hash::{Hash, Hasher};

use crate::*;

pub struct Id<G: GenericHasher>(G::Digest);

impl<G: GenericHasher> Hash for Id<G> where G::Digest: Hash {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state)
	}
}

impl<G: GenericHasher> PartialEq for Id<G> where G::Digest: PartialEq {
	fn eq(&self, Id(digest): &Self) -> bool {
		self.0.eq(&digest)
	}
}

impl<G: GenericHasher> Eq for Id<G> where G::Digest: Eq {}

pub enum Entry<G: GenericHasher> {
	First {
		digest: Id<G>,
		size: u64,
	},
	Suc {
		digest: Id<G>,
		size: u64,
		seqno: u64,
	
		pred: Id<G>,
		skip: Id<G>,

		pred_delta_size: u64,
		skip_delta_size: u64,
	},
}
