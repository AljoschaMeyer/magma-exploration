use async_trait::async_trait;

// forall x: Self. Self::concat(&Self::neutral(), &x) == &x
// forall x: Self. Self::concat(&x, &Self::neutral()) == &x
#[async_trait]
pub trait AsyncUnitalMagma {
	fn neutral() -> Self;
	async fn concat(x: &Self, y: &Self) -> Self;
}

// forall x, y, z: Self. Self::concat(Self::concat(x, y), z) == Self::concat(x, Self::concat(y, z))
#[async_trait]
pub trait AsyncMonoid: AsyncUnitalMagma {}

pub trait GenericHasher {
	type Digest;

	fn finish(&self) -> Self::Digest;
	fn write(&mut self, bytes: &[u8]);
}

pub struct Id<H: GenericHasher>(H::Digest);

pub enum Entry<H: GenericHasher> {
	First {
		digest: Id<H>,
		size: u64,
	},
	Suc {
		digest: Id<H>,
		size: u64,
		seqno: u64,
	
		pred: Id<H>,
		skip: Id<H>,

		pred_delta_size: u64,
		skip_delta_size: u64,
	},
}
