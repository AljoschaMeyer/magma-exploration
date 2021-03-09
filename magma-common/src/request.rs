use crate::*;

pub struct Request<G: GenericHasher> {
	id: Id<G>,
	origin: Option<Id<G>>,
	min_gran: Granularity,
	max_gran: Granularity,
}

enum Granularity {
	Direct, // give me only the payload of `id` -- 1 payload
	SkipLinks, // -- O(log n) payloadpayloads
	PredLinks, // -- O(n) payloads
}
