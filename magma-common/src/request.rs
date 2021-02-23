use crate::*;

pub struct Request<G: GenericHasher> {
	id: Id<G>,
	origin: Option<Id<G>>,
}
