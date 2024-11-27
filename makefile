pi:
	# cross build --release --target armv7-unknown-linux-gnueabihf
	# scp ../target/armv7-unknown-linux-gnueabihf/release/doorsensor pi@10.0.0.187:~/
	# ssh -t pi@10.0.0.187 "./doorsensor"
	ssh -t pi@10.0.0.187 "cd ~/doorsensor && git pull origin master && /home/pi/.cargo/bin/cargo build --release && ./target/release/doorsensor"