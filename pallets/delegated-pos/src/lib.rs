#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits:: {
		Currency,
		ReservableCurrency,
		ConstU32
	};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MyToken: ReservableCurrency<Self::AccountId>;
		///type ReservationFee: Get<<<Self as Config>::MyToken as Currency<<Self as Config>::AccountId>>::Balance>;
		type ForceOrigin: EnsureOrigin<Self::Origin>;

		///The minimun amount one can delegate to avoid spam attacks
		type MinDelegateAmount: Get<U32>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]

	/// Key: Delegate, Value: Vec<(Sender, Amount)>
	pub type DelegatedTokens<T: Config> = StorageMap<T::AccoundId, Blake2_128Concat, T::AccountId, Get<U64>, ValueQuery>;
	
	/// Key: DelegateID, UnitType
	/// must have either a BABE session key or voted on by governance.
	pub type IsDelegatable<T: Config> = StorageMap<(), Blake2_128Concat, T::AccountId, (), ValueQuery>;
	pub type Nominators<T: Config> = StorageValue<(), Blake2_128Concat, T::AccountId, (), ValueQuery>;
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		StakeChange(T::AccountId),
		DelegatedFunds(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotEnoughFunds,
		ValidatorMaxStake,
		BelowMinimumAmount,
		NotDelegatable,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Delegate amount of tokens to a user who is a known delegate or validator.
		/// Known delegators can only delegate to validators.
		pub fn delegate_tokens(origin: OriginFor<T>, delegate: T::AccountId, amount: Get<U64>) -> DispatchResult {
			// Ensure that :
			// Sender is legit
			// The recipient is Delegatable (either voted and known or a validator). 
			// The sender has enough funds.
			// The recipient is delegatable (either voted on or a validator).
			let sender = ensure_signed(origin)?;
			ensure!(amount > T::MinDelegateAmount, Error::<T>::BelowMinimumAmount);
			ensure!(T::MyToken::can_reserve(origin::OriginFor<T>::AccoundId, amount), Error::<T>::NotEnoughFunds);
			ensure!(T::IsDelegatable.contains_key(delegate), Error::<T>::NotDelegatable);

			T::MyToken::reserve(&sender, amount.into()).expect("ensure reserve amount has been called. qed");

			 //Send equal amount also to DelegatedTokens
			 //todo: find out what to store this as so that we can do DELEGATEID -> SENDERID = amount.
			AmountDelegated::<T>::set(
				delegate,
				amount
			);
			
			Self::deposit_event(Event::ClaimCreated(sender, proof));
			Ok(())
		} 

		pub fn revoke_delegation(origin: OriginFor<T>, delegate: T::AccountId, amount: Get<U64>) -> DispatchResult {
			unimplemented!();
		}

		pub revoke_delegation_all(origin: OriginFor<T>) -> DispatchResult {
			unimplemented!();
		}

		pub fn auto_delegate_validators(origin: OriginFor<T>) {
			unimplemented!();
		}

		pub fn make_delegatable(origin: OriginFor<T>, delegate: T::AccoundId) {
			unimplemented!();
		}
		
		pub fn revoke_delegatable(delegate: T::AccoundId) {
			unimplemented!();
		}

		/* #[pallet::weight(1_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			proof: Vec<u8>,
			// proof: BoundedVec<u8, T::HashLimit>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			T::MyToken::reserve(&sender, 1000u32.into())?;

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, proof));
			Ok(())
		}
 */
	}
}
