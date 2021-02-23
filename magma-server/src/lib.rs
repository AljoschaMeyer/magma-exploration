use magma_common::*;

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use async_trait::async_trait;
use futures::Stream;

#[async_trait(?Send)]
pub trait Storage<G: GenericHasher, T> {
	async fn get<'s>(&'s self, id: &Id<G>) -> Option<&'s T>;
	async fn get_entry(&self, id: &Id<G>) -> Entry<G>;
}

pub struct Server<G: GenericHasher, T, S: Storage<G, T>> {
	storage: S,
	_phantom: PhantomData<(G, T)>,
}

impl<G: GenericHasher, T, S: Storage<G, T>> Server<G, T, S> {
	fn reply(request: Request<G>) -> impl Stream<Item=T> {
		// TODO
	}
}

struct InMemory<G: GenericHasher, T>(HashMap<Id<G>, T>);

#[async_trait(?Send)]
impl<G: GenericHasher, T> Storage<G, T> for InMemory<G, T> where G::Digest: Hash + Eq {
	async fn get<'s>(&'s self, id: &Id<G>) -> Option<&'s T> {
		self.0.get(id)
	}

	async fn get_entry(&self, id: &Id<G>) -> Entry<G> {
		unimplemented!()
	}
}
