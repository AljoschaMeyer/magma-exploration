use crate::*;

// forall x: Self. Self::concat(&Self::neutral(), &x) == &x
// forall x: Self. Self::concat(&x, &Self::neutral()) == &x
#[async_trait(?Send)]
pub trait AsyncUnitalMagma {
	fn neutral() -> Self;
	async fn concat(x: &Self, y: &Self) -> Self;
}

// forall x, y, z: Self. Self::concat(Self::concat(x, y), z) == Self::concat(x, Self::concat(y, z))
#[async_trait(?Send)]
pub trait AsyncMonoid: AsyncUnitalMagma {}
