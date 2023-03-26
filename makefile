pi:
	cross build --release --target armv7-unknown-linux-gnueabihf
	scp ../target/armv7-unknown-linux-gnueabihf/release/doorsensor pi@10.0.0.188:~/
	ssh -t pi@10.0.0.188 "./doorsensor"