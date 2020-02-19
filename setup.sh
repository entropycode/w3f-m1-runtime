#!/usr/bin/env bash

set -ex

git clone -b pre-v2.0-3e65111 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template node
git checkout -b feedback-node

lib='node/runtime/src/lib.rs'
cargotoml='node/runtime/Cargo.toml'


# sed for OSX and Linux is different, OSX requires extension to be specified
if [[ "$OSTYPE" == "darwin"* ]]; then

	echo 'Darwin'
	
	sed -i '' 's/construct_runtime!(/impl feedback::Trait for Runtime {\
		type Event = Event;\
	}\
	\
	construct_runtime!(/' $lib

	sed -i '' 's/Sudo: sudo,/Sudo: sudo,\
			Feedback: feedback::{Module, Call, Storage, Event<T>},/' $lib

	sed -i '' "s/\[features\]/[dependencies.feedback]\\
	default-features = false\\
	path = '..\/..\/feedback'\\
	\\
	[features]/" $cargotoml

	sed -i '' "s/'transaction-payment\/std',/'transaction-payment\/std',\\
		'feedback\/std'/" $cargotoml

elif [[ "$OSTYPE" == "linux-gnu" ]]; then

	echo 'Linux-GNU'

	sed -i 's/construct_runtime!(/impl feedback::Trait for Runtime {\
		type Event = Event;\
	}\
	\
	construct_runtime!(/' $lib

	sed -i 's/Sudo: sudo,/Sudo: sudo,\
			Feedback: feedback::{Module, Call, Storage, Event<T>},/' $lib

	sed -i "s/\[features\]/[dependencies.feedback]\\
	default-features = false\\
	path = '..\/..\/feedback'\\
	\\
	[features]/" $cargotoml

	sed -i "s/'transaction-payment\/std',/'transaction-payment\/std',\\
		'feedback\/std'/" $cargotoml

fi

