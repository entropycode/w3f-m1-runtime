#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult, dispatch::Vec, StorageValue, StorageMap, ensure};
use system::ensure_signed;
use codec::{Encode, Decode};
use timestamp;
use sp_runtime::traits::{Hash, CheckedAdd};

pub trait Trait: system::Trait  + timestamp::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Poll <Hash, Moment> {
	title: Vec<u8>,
	options_hashes: Vec<Hash>,
	choice: Hash,
	expiration: Moment
}

decl_storage! {
	trait Store for Module<T: Trait> as Feedback {
		PollId get(poll_id): u64;
		Polls get(polls): map u64 => Poll<T::Hash, T::Moment>;
		Entries get(entries): map (T::AccountId, u64) => bool;
		Responses get(responses): map (u64, T::Hash) => u64;
	}
}

// The pallet's dispatchable functions.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn poll(origin, _title: Vec<u8>, _options: Vec<Vec<u8>>, _open_for: T::Moment) -> DispatchResult {
			ensure_signed(origin)?;

			let poll_id = PollId::get().checked_add(1).ok_or("Overflow occurred at poll id")?;
			PollId::put(poll_id);

			let now = <timestamp::Module<T>>::get();
			
			let mut options_hashes = Vec::new();
			for opt in _options.iter() {
				options_hashes.push(<T as system::Trait>::Hashing::hash_of(opt));
			}

			let new_poll = Poll {
				title: _title,
				options_hashes: options_hashes,
				choice: <T as system::Trait>::Hashing::hash_of(&0),
				expiration: now.checked_add(&_open_for).ok_or("Overflow occured at setting poll expiration")?
			};

			<Polls<T>>::insert(poll_id, new_poll.clone());
			
			Self::deposit_event(RawEvent::NewPollStored(poll_id, _options, new_poll.expiration));

			Ok(())
		}

		pub fn respond(origin, _poll_id: u64, _entry: Vec<u8> ) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<Polls<T>>::exists(_poll_id), "Must respond to an existing poll");
			ensure!(!<Entries<T>>::get((who.clone(), _poll_id)), "Account has responded for this poll");

			let now = <timestamp::Module<T>>::get();
			let poll = <Polls<T>>::get(_poll_id);
			ensure!( poll.expiration > now, "Poll has expired");

			let entry_hash = <T as system::Trait>::Hashing::hash_of(&_entry);

			let response = <Responses<T>>::get((_poll_id, entry_hash)).checked_add(1).ok_or("Overflow occurred at response")?;
			<Responses<T>>::insert((_poll_id, entry_hash), response);
			<Entries<T>>::insert((who.clone(), _poll_id), true);
			Self::deposit_event(RawEvent::NewRespond(_poll_id, entry_hash));

			Ok(())
		}

		pub fn seal(_origin, _poll_id: u64) -> DispatchResult {
			
			ensure!(<Polls<T>>::exists(_poll_id), "Poll does not exist");
			let poll = <Polls<T>>::get(_poll_id);
			let now = <timestamp::Module<T>>::get();
			ensure!( poll.expiration < now, "Poll is still active");
			let mut popular_response = 0;

			for opt in poll.options_hashes.iter() {
				if <Responses<T>>::get((_poll_id, opt.clone())) > popular_response {
					popular_response = <Responses<T>>::get((_poll_id, opt.clone()));
					<Polls<T>>::mutate(_poll_id, |poll| {poll.choice = opt.clone()});
				} else if <Responses<T>>::get((_poll_id, opt.clone())) == popular_response {
					<Polls<T>>::mutate(_poll_id, |poll| {poll.choice = <T as system::Trait>::Hashing::hash_of(&0)});
				}
			}

			Self::deposit_event(RawEvent::Choice(_poll_id, <Polls<T>>::get(_poll_id).choice));

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T>
	where Hash = <T as system::Trait>::Hash, Moment = <T as timestamp::Trait>::Moment
	{
		NewPollStored(u64, Vec<Vec<u8>>, Moment),
		NewRespond(u64, Hash),
		Choice(u64, Hash),
	}
);

/// tests for this pallet
#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::H256;
	use frame_support::{impl_outer_origin, assert_ok, assert_noop, parameter_types, weights::Weight};
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the pallet, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
		type ModuleToIndex = ();
	}
	impl Trait for Test {
		type Event = ();
	}

	impl timestamp::Trait for Test {
		type Moment = u64;
		type OnTimestampSet = ();
		type MinimumPeriod = ();
	}

	type Feedback = Module<Test>;
	type Moment = timestamp::Module<Test>;


	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> sp_io::TestExternalities {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn new_poll_works_for_default_value() {
		new_test_ext().execute_with(|| {
			let title = vec![123];
			let option = vec![1];
			let options = vec![option.clone(), option.clone()];
			let option_hash = <tests::Test as system::Trait>::Hashing::hash_of(&option.clone());
			let options_hashes = vec![option_hash, option_hash];
			let poll_creator = 1;
			let expires_in = 240;
			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			assert_eq!(Feedback::poll_id(), 1);
			assert_eq!(Feedback::polls(1).title, title);
			assert_eq!(Feedback::polls(1).options_hashes, options_hashes);
			assert_eq!(Feedback::polls(1).expiration, expires_in);
		});
	}

	#[test]
	fn first_respond_for_a_poll_works() {
		new_test_ext().execute_with(|| {
			let title = vec![123];
			let option = vec![1];
			let option_hash = <tests::Test as system::Trait>::Hashing::hash_of(&option.clone());
			let options = vec![option.clone(), option.clone() ];
			let poll_creator = 1;
			let respond_poll_id = 1;
			let responder = 2;
			let expires_in = 240;

			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			assert_ok!(Feedback::respond(Origin::signed(responder), respond_poll_id, option.clone()));
			assert_eq!(Feedback::entries((responder, respond_poll_id)), true);
			assert_eq!(Feedback::responses((respond_poll_id, option_hash)), 1);
		});
	}

	#[test]
	fn respond_to_nonexisting_poll_fails() {
		new_test_ext().execute_with(|| {
			let option = vec![1];
			let respond_poll_id = 1;
			let responder = 2;
			assert_noop!(Feedback::respond(Origin::signed(responder), respond_poll_id, option.clone()), 
				"Must respond to an existing poll");
		});
	}

	#[test]
	fn respond_to_expired_poll_fails() {
		new_test_ext().execute_with(|| {
			let title = vec![123];
			let option = vec![1];
			let options = vec![option.clone(), option.clone() ];
			let poll_creator = 1;
			let respond_poll_id = 1;
			let responder = 2;
			let expires_in = 120;
			
			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			Moment::set_timestamp(129);
			assert_noop!(Feedback::respond(Origin::signed(responder), respond_poll_id, option.clone()), "Poll has expired");
		});
	}

	#[test]
	fn second_for_responded_poll_fails() {
		new_test_ext().execute_with(|| {
			let title = vec![123];
			let option = vec![1];
			let options = vec![option.clone(), option.clone()];
			let poll_creator = 1;
			let respond_poll_id = 1;
			let responder = 2;
			let expires_in = 240;

			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			assert_ok!(Feedback::respond(Origin::signed(responder), respond_poll_id, option.clone()));
			assert_noop!(Feedback::respond(Origin::signed(responder), respond_poll_id, option.clone()), 
				"Account has responded for this poll");
		});
	}

	#[test]
	fn popular_choice_set_works() {
		new_test_ext().execute_with(|| {

			let title = vec![123];
			let option1 = vec![1];
			let option2 = vec![2];
			let option1_hash = <tests::Test as system::Trait>::Hashing::hash_of(&option1.clone());
			let options = vec![option1.clone(), option2.clone()];
			let poll_creator = 1;
			let respond_poll_id = 1;
			let responder1 = 2;
			let responder2 = 3;
			let expires_in = 240;

			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			assert_ok!(Feedback::respond(Origin::signed(responder1), respond_poll_id, option1.clone()));
			assert_ok!(Feedback::respond(Origin::signed(responder2), respond_poll_id, option1.clone()));
			Moment::set_timestamp(241);
			assert_ok!(Feedback::seal(Origin::signed(responder2), respond_poll_id));
			assert_eq!(Feedback::polls(respond_poll_id).choice, option1_hash);
		});
	}

	#[test]
	fn empty_choice_for_draw_works() {
		new_test_ext().execute_with(|| {

			let title = vec![123];
			let option1 = vec![1];
			let option2 = vec![2];
			let options = vec![option1.clone(), option2.clone()];
			let poll_creator = 1;
			let respond_poll_id = 1;
			let responder1 = 2;
			let responder2 = 3;
			let expires_in = 240;
			let empty_hash = <tests::Test as system::Trait>::Hashing::hash_of(&0);


			assert_ok!(Feedback::poll(Origin::signed(poll_creator), title.clone(), options, expires_in));
			assert_ok!(Feedback::respond(Origin::signed(responder1), respond_poll_id, option1.clone()));
			assert_ok!(Feedback::respond(Origin::signed(responder2), respond_poll_id, option2.clone()));
			Moment::set_timestamp(241);
			assert_ok!(Feedback::seal(Origin::signed(responder2), respond_poll_id));
			assert_eq!(Feedback::polls(respond_poll_id).choice, empty_hash);
		});
	}


}
