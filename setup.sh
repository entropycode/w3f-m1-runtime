#!/bin/sh

# Cloning node template
git clone -n https://github.com/substrate-developer-hub/substrate-node-template.git node
cd node/
git checkout tags/pre-v2.0-3e65111 -b node

lib='runtime/src/lib.rs'
cargotoml='runtime/Cargo.toml'

# Adding impl of feedback to runtime/lib.rs
sed -i '' 's/construct_runtime!(/impl feedback::Trait for Runtime {\
	type Event = Event;\
}\
\
construct_runtime!(/' $lib

# Include in construct_runtime! in runtime/lib.rs
sed -i '' 's/Sudo: sudo,/Sudo: sudo,\
		Feedback: feedback::{Module, Call, Storage, Event<T>},/' $lib

# Include feedback dependency in cargo.toml
sed -i '' "s/\[features\]/[dependencies.feedback]\\
default-features = false\\
path = '..\/..\/feedback'\\
\\
[features]/" $cargotoml

# std Features in cargo.toml
sed -i '' "s/'transaction-payment\/std',/'transaction-payment\/std',\\
    'feedback\/std'/" $cargotoml

cargo build --release