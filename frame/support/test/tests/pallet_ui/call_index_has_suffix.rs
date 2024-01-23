#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::DispatchResultWithPostInfo;
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0something)]
		pub fn foo(origin: OriginFor<T>) -> DispatchResultWithPostInfo {}
	}
}

fn main() {
}
