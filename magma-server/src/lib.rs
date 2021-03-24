#![feature(never_type)]
#![feature(generators, generator_trait)]

use magma_common::*;

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::io::Write;
use std::ops::{Generator, GeneratorState};

use async_trait::async_trait;
use futures::Stream;

#[async_trait(?Send)]
pub trait Storage<G: GenericHasher, E> {
	type Writer: Write;

	fn get<'s>(&'s self, id: &Id<G>, offset: u64) -> Result<Option<Box<[u8]>>, E>;
	fn get_entry(&self, id: &Id<G>) -> Result<Option<Entry<G>>, E>;
}

pub struct Server<G: GenericHasher, E, S: Storage<G, E>> {
	storage: S,
	_phantom: PhantomData<(G, E)>,
}

fn entry_path<G: GenericHasher>(request: &Request<G>) -> impl Iterator<Item=Id<G>> {
	panic!("TODO !!!");
	vec!().into_iter() // TODO
}

struct ServerIter<G: GenericHasher, E, S: Storage<G, E>> {
	request: Request<G>,
	_phantom: PhantomData<(G, E, S)>,
	/* TODO internal state */
}

impl<G: GenericHasher, E, S: Storage<G, E>> ServerIter<G, E, S> {
	fn new(request: Request<G>) -> Self {
		Self {
			request,
			_phantom: PhantomData,
		}
	}
}

impl<G: GenericHasher, E, S: Storage<G, E>> Iterator for ServerIter<G, E, S> {
	type Item = Result<Option<Box<[u8]>>, E>;

	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!()
	}
}

impl<G: GenericHasher, E, S: Storage<G, E>> Server<G, E, S> {
	fn reply(&self, request: Request<G>) -> impl Generator<Yield=Entry<G>, Return=Result<Option<ServerIter<G, E, S>>, E>> + '_ {
		let mut gen = move || {
			for id in entry_path(&request) {
				match self.storage.get_entry(&id) {
					Ok(Some(entry)) => yield entry,
					Ok(None) => return Ok(None),
					Err(e) => return Err(e),
				}
			}

			return Ok(Some(ServerIter::new(request)));
		};
		gen
	}
}

struct InMemory<G: GenericHasher>(HashMap<Id<G>, Box<[u8]>>);

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
