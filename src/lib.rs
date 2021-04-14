//! # $name$
//!
//! - [`$name_pascal_case$::Config`](./trait.Config.html)
//!
//! ## Overview
//!
//! $param.description$
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `$param.method1_snake_case$` -
//! * `$param.method2_snake_case$` -
//! * `$param.method3_snake_case$` -
//!

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchError,
    ensure,
    traits::{Currency, EnsureOrigin, Get, OnUnbalanced, ReservableCurrency, UnixTime},
};
use frame_system::ensure_signed;
use sp_runtime::RuntimeDebug;
use sp_runtime::{
    traits::{StaticLookup, Zero},
    SaturatedConversion,
};
use sp_std::{fmt::Debug, prelude::*, vec};

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

use codec::{Decode, Encode, HasCompact};

type $param.object_name_pascal_case$Id = u32;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The origin which may forcibly set or remove a name. Root can always do this.
        type ForceOrigin: EnsureOrigin<Self::Origin>;

        /// Min $param.object_name_lower_case$ name length
        type Min$param.object_name_pascal_case$NameLength: Get<usize>;

        /// Max $param.object_name_lower_case$ name length
        type Max$param.object_name_pascal_case$NameLength: Get<usize>;
        
        /// Weight information
        type WeightInfo: WeightInfo;
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug)]
    pub struct $param.object_name_pascal_case$<AccountId: Encode + Decode + Clone + Debug + Eq + PartialEq> {
        /// $param.object_name$ name
        name: Vec<u8>,

        /// owner of the $param.object_name$
        owner: AccountId,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The $param.object_name$ already exsits
        AlreadyExists,

        /// Name too long
        TooLong,

        /// Name too short
        TooShort,

        /// $param.object_name$ doesn't exist.
        NotExists,

        /// Origin has no authorization to do this operation
        PermissionDenied,

        /// ID already exists
        IdAlreadyExists,

        /// Unknown error occurred
        Unknown,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance", $param.object_name_pascal_case$Id = "$param.object_name_pascal_case$Id")]
    pub enum Event<T: Config> {
        /// Some $param.object_name$ added inside the system.
        $param.object_name_pascal_case$Added($param.object_name_pascal_case$Id, T::AccountId),

        /// When $param.object_name$ deleted
        $param.object_name_pascal_case$Deleted($param.object_name_pascal_case$Id)
    }

    /// Index of id -> data
    #[pallet::storage]
    pub type $param.object_name_pascal_case$s<T: Config> =
        StorageMap<_, Blake2_128Concat, $param.object_name_pascal_case$Id, $param.object_name_pascal_case$<T::AccountId>>;

    #[pallet::storage]
    pub type $param.object_name_pascal_case$Link<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        $param.object_name_pascal_case$Id,
        u32, // change me
    >;

    #[pallet::storage]
    pub type $param.object_name_pascal_case$IdIndex<T> = StorageValue<_, u32>;

    /// $name_pascal_case$ module declaration.
    // pub struct Module<T: Config> for enum Call where origin: T::Origin {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add new object.
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// # <weight>
        /// # </weight>
        #[pallet::weight(T::WeightInfo::$param.method1_snake_case$())]
        fn $param.method1_snake_case$(
            origin: OriginFor<T>,
            name: Vec<u8>,
            owner: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResultWithPostInfo {
            let _origin = T::ForceOrigin::ensure_origin(origin)?;

            ensure!(
                name.len() >= T::Min$param.object_name_pascal_case$NameLength::get(),
                Error::<T>::TooShort
            );
            ensure!(
                name.len() <= T::Max$param.object_name_pascal_case$NameLength::get(),
                Error::<T>::TooLong
            );

            let id = Self::next_id();

            ensure!(
                !$param.object_name_pascal_case$s::<T>::contains_key(id),
                Error::<T>::IdAlreadyExists
            );

            let owner = T::Lookup::lookup(owner)?;

            $param.object_name_pascal_case$s::<T>::insert(
                id as $param.object_name_pascal_case$Id,
                $param.object_name_pascal_case$ {
                    name: name.clone(),
                    owner: owner.clone(),
                },
            );

            Self::deposit_event(Event::$param.object_name_pascal_case$Added(id, owner));

            Ok(().into())
        }

        /// $param.method2$
        /// 
        #[pallet::weight(100_000)]
        fn $param.method2_snake_case$(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

        /// $param.method3$
        /// 
        #[pallet::weight(100_000)]
        fn $param.method3_snake_case$(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }

    }

    // ----------------------------------------------------------------
    //                      HOOKS
    // ----------------------------------------------------------------
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        // fn offchain_worker(n: T::BlockNumber){
        //     // @TODO(you): Your off-chain logic here
        // }
    }


    // -------------------------------------------------------------------
    //                      GENESIS CONFIGURATION
    // -------------------------------------------------------------------

	// The genesis config type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub dummy: u32,
		pub bar: Vec<(T::AccountId, u32)>,
		pub foo: u32,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				dummy: Default::default(),
				bar: Default::default(),
				foo: Default::default(),
			}
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// <Dummy<T>>::put(&self.dummy);
			// for (a, b) in &self.bar {
			// 	<Bar<T>>::insert(a, b);
			// }
			// <Foo<T>>::put(&self.foo);
		}
	}
}

/// The main implementation of this $name_pascal_case$ pallet.
impl<T: Config> Pallet<T> {
    /// Get the $name$ detail
    pub fn $name_snake_case$(id: $param.object_name_pascal_case$Id) -> Option<$param.object_name_pascal_case$<T::AccountId>> {
        $param.object_name_pascal_case$s::<T>::get(id)
    }

    /// Get next $name$ ID
    pub fn next_id() -> u32 {
        let next_id = <$param.object_name_pascal_case$IdIndex<T>>::try_get().unwrap_or(0).saturating_add(1);
        <$param.object_name_pascal_case$IdIndex<T>>::put(next_id);
        next_id
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as pallet_$name_snake_case$;

    use frame_support::{assert_noop, assert_ok, ord_parameter_types, parameter_types};
    use frame_system::EnsureSignedBy;
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };

    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Module, Call, Config, Storage, Event<T>},
            Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
            $name_pascal_case$: pallet_$name_snake_case$::{Module, Call, Storage, Event<T>},
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub BlockWeights: frame_system::limits::BlockWeights =
            frame_system::limits::BlockWeights::simple_max(1024);
    }
    impl frame_system::Config for Test {
        type BaseCallFilter = ();
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = Origin;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Call = Call;
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
        type SS58Prefix = ();
    }
    parameter_types! {
        pub const ExistentialDeposit: u64 = 1;
    }
    impl pallet_balances::Config for Test {
        type MaxLocks = ();
        type Balance = u64;
        type Event = Event;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
    }
    parameter_types! {
        pub const Min$param.object_name_pascal_case$NameLength: usize = 3;
        pub const Max$param.object_name_pascal_case$NameLength: usize = 16;
    }
    ord_parameter_types! {
        pub const One: u64 = 1;
    }
    impl Config for Test {
        type Event = Event;
        type ForceOrigin = EnsureSignedBy<One, u64>;
        type Min$param.object_name_pascal_case$NameLength = Min$param.object_name_pascal_case$NameLength;
        type Max$param.object_name_pascal_case$NameLength = Max$param.object_name_pascal_case$NameLength;
        type WeightInfo = weights::SubstrateWeight<Test>;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(1, 10), (2, 10)],
        }
        .assimilate_storage(&mut t)
        .unwrap();
        t.into()
    }

    #[test]
    fn force_origin_able_to_create_$param.object_name_snake_case$() {
        new_test_ext().execute_with(|| {
            assert_ok!($name_pascal_case$::$param.method1_snake_case$(
                Origin::signed(1), b"ORG1".to_vec(), 2));
        });
    }

    #[test]
    fn non_force_origin_cannot_create_$param.object_name_snake_case$() {
        new_test_ext().execute_with(|| {
            assert_noop!($name_pascal_case$::$param.method1_snake_case$(
                Origin::signed(2), b"ORG1".to_vec(), 2
            ), DispatchError::BadOrigin);
        });
    }

}
