use crate::{self as simple_crowdfund, Config};
use super::*;

use frame_support::{parameter_types};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Percent, Permill,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		SimpleCrowdfund: simple_crowdfund::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub BlockWeights: frame_system::limits::BlockWeights =
	frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
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
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type MaxLocks = ();
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: u64 = 1;
	pub const SpendPeriod: u64 = 2;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: u64 = 1;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: u64 = 1;
	pub const TipReportDepositPerByte: u64 = 1;
}

// EXPLA: added simple crowdfund pallet parameter here and configured it
parameter_types! {
	pub const SubmissionDeposit: u64 = 1;
	pub const MinContribution: u64 = 10;
	pub const RetirementPeriod: u64 = 5;
}

impl Config for Test {
	type Event = Event;
	/// The currency in which the crowdfunds will be denominated
	type Currency = Balances;
	/// The amount to be held on deposit by the owner of a crowdfund
	type SubmissionDeposit = SubmissionDeposit;
	/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
	/// least ExistentialDeposit.
	type MinContribution = MinContribution;
	/// The period of time (in blocks) after an unsuccessful crowdfund ending during which
	/// contributors are able to withdraw their funds. After this period, their funds are lost.
	type RetirementPeriod = RetirementPeriod;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	// EXPLA: looks like a view bug of VS, ignore it that the GenesisConfig would not be found... it is found!
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 1000), (2, 2000), (3, 3000), (4, 4000)],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}

use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnInitialize;
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OnFinalize;

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		SimpleCrowdfund::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		SimpleCrowdfund::on_initialize(System::block_number());
	}
}