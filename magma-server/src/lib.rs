#![feature(never_type)]
#![feature(generators, generator_trait)]

use magma_common::*;

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::io::Write;

use async_trait::async_trait;
use futures::Stream;

#[async_trait(?Send)]
pub trait Storage<G: GenericHasher, T, E> {
	type Writer: Write;

	fn get<'s>(&'s self, id: &Id<G>, offset: u64) -> Result<Option<&'s T>, E>;
	fn get_entry(&self, id: &Id<G>) -> Result<Option<Entry<G>>, E>;
}

pub struct Server<G: GenericHasher, T, E, S: Storage<G, T, E>> {
	storage: S,
	_phantom: PhantomData<(G, T, E)>,
}

fn entry_path<G: GenericHasher>(request: &Request<G>) -> impl Iterator<Item=Id<G>> {
	unimplemented!()
}

fn result_path<G: GenericHasher>(request: &Request<G>) -> impl Iterator<Item=Id<G>> {
	unimplemented!()
}

impl<G: GenericHasher, T, E, S: Storage<G, T, E>> Server<G, T, E, S> {
	type Iter = Iterator<Item=Result<Option<T>, S::E>>;

	fn reply(&self, request: Request<G>) -> impl Generator<Yield=Entry<G>, Return=Result<Option<Iter>, S::E>> {
		let mut gen = || {
			for id in entry_path(&request) {
				match self.get_entry(&id) {
					Ok(Some(entry)) => {
						yield entry,
					}
					Ok(None) => return Ok(None),
					Err(e) => return Err(e),
				}
			}
			return Ok(Some(
				result_path(&request).map(|x| )
			));
		};
		gen
	}
}

struct InMemory<G: GenericHasher, T>(HashMap<Id<G>, T>);

/*
#[async_trait(?Send)]
impl<G: GenericHasher, T> Storage<G, T, !> for InMemory<G, T> where G::Digest: Hash + Eq {
	async fn get<'s>(&'s self, id: &Id<G>) -> Result<Option<&'s T>, !> {
		Ok(self.0.get(id))
	}

	async fn get_entry(&self, id: &Id<G>) -> Result<Option<Entry<G>>, !> {
		unimplemented!()
	}
}
*/
