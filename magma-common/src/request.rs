use crate::*;

enum Offset {
	Entries(/*sequence_num:*/ u64), // only send the entries with sequence number <= sequence_num
	BytesWithinPayload(u64),
}

enum Granularity {
	// n = distance between id and origin (i.e. id - origin)
	Direct, // give me only the payload of `id` -- 1 payload
	SkipLinks, // -- O(log n) payloads
	PredLinks, // -- O(n) payloads
}

pub struct Request<G: GenericHasher> {
	id: Id<G>,
	origin: Option<Id<G>>,
	min_gran: Granularity,
	max_gran: Granularity,
	offset: Offset,
}
