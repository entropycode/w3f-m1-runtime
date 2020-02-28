#!/usr/bin/env bash

set -ex

git clone -b pre-v2.0-3e65111 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template node

lib='node/runtime/src/lib.rs'
cargotoml='node/runtime/Cargo.toml'
roottoml='node/Cargo.toml'


# sed for OSX and Linux is different, OSX requires extension to be specified
if [[ "$OSTYPE" == "darwin"* ]]; then

	echo 'Darwin'

	sed -i '' 's/mod template;//' $lib
	
	sed -i '' 's/template::/feedback::/' $lib

	sed -i '' 's/TemplateModule:/Feedback:/' $lib

	sed -i '' 's/node-template/feedback-node/' $lib

	sed -i '' "s/\[features\]/[dependencies.feedback]\\
	default-features = false\\
	path = '..\/..\/feedback'\\
	\\
	[features]/" $cargotoml

	sed -i '' "s/'transaction-payment\/std',/'transaction-payment\/std',\\
		'feedback\/std'/" $cargotoml

	
	sed -i '' "s/name = 'node-template'/name = 'feedback-node'/" $roottoml

elif [[ "$OSTYPE" == "linux-gnu" ]]; then

	echo 'Linux-GNU'


	sed -i 's/mod template;//' $lib
	
	sed -i 's/template::/feedback::/' $lib

	sed -i 's/TemplateModule:/Feedback:/' $lib

	sed -i 's/node-template/feedback-node/' $lib

	sed -i "s/\[features\]/[dependencies.feedback]\\
	default-features = false\\
	path = '..\/..\/feedback'\\
	\\
	[features]/" $cargotoml

	sed -i "s/'transaction-payment\/std',/'transaction-payment\/std',\\
		'feedback\/std'/" $cargotoml

	
	sed -i "s/name = 'node-template'/name = 'feedback-node'/" $roottoml

fi

