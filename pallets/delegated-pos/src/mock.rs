use crate as delegated_pos;
use frame_support::traits::{ConstU16, ConstU64};
use frame_system as system;
use pallet_balances::*;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
use frame_support::traits::{GenesisBuild, ReservableCurrency};
use frame_system::EnsureRoot;
use sp_runtime::traits::{ConstU32, Get};

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Dpos: delegated_pos,
		Balances: pallet_balances,
	}
);

type Balance = u64;

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

impl delegated_pos::Config for Test {
	type Event = Event;
	type MyToken = Balances; //TODO>???
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type MinimumStake = ConstU64<500>;
	type BlocksTillSwap = ConstU64<1>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(0, 1000000), (1, 9999999), (2, 10000000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	crate::GenesisConfig::<Test> {
		init_validators: (0..50).collect::<Vec<u64>>(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
