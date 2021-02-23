use crate::*;

pub trait GenericHasher {
	type Digest;

	fn finish(&self) -> Self::Digest;
	fn write(&mut self, bytes: &[u8]);
}

#[async_trait]
pub trait AsyncHash {
	async fn hash<G: GenericHasher>(&self, state: &mut G);
}
