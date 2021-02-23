use crate::*;

pub struct Id<G: GenericHasher>(G::Digest);

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

