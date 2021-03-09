#![feature(never_type)]

use magma_common::*;

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use async_trait::async_trait;
use futures::Stream;

#[async_trait(?Send)]
pub trait Storage<G: GenericHasher, T, E> {
	async fn get<'s>(&'s self, id: &Id<G>) -> Result<Option<&'s T>, E>;
	async fn get_entry(&self, id: &Id<G>) -> Result<Option<Entry<G>>, E>;
}

pub struct Server<G: GenericHasher, T, E, S: Storage<G, T, E>> {
	storage: S,
	_phantom: PhantomData<(G, T, E)>,
}

impl<G: GenericHasher, T, E, S: Storage<G, T, E>> Server<G, T, E, S> {
	fn reply(request: Request<G>) -> impl Stream<Item=T> {
		futures::stream::empty() // TODO
	}
}

struct InMemory<G: GenericHasher, T>(HashMap<Id<G>, T>);

#[async_trait(?Send)]
impl<G: GenericHasher, T> Storage<G, T, !> for InMemory<G, T> where G::Digest: Hash + Eq {
	async fn get<'s>(&'s self, id: &Id<G>) -> Result<Option<&'s T>, !> {
		Ok(self.0.get(id))
	}

	async fn get_entry(&self, id: &Id<G>) -> Result<Option<Entry<G>>, !> {
		unimplemented!()
	}
}
