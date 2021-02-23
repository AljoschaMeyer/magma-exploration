use magma_common::*;

use std::collections::HashMap;
use std::hash::Hash;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait Storage<G: GenericHasher, T> {
	async fn get<'s>(&'s self, id: &Id<G>) -> Option<&'s T>;
}

/*
pub struct Server<S: Storage> { }
*/

struct InMemory<G: GenericHasher, T>(HashMap<Id<G>, T>);

#[async_trait(?Send)]
impl<G: GenericHasher, T> Storage<G, T> for InMemory<G, T> where G::Digest: Hash + Eq {
	async fn get<'s>(&'s self, id: &Id<G>) -> Option<&'s T> {
		self.0.get(id)
	}
}
