run_sample:
	RUST_LOG=info cargo run "DarmakzetrekurukzetresamavetAyuyutsavas;mAmakAHpARqavAScEvakimakurvatasaMjaya"

check_memory:
	RUST_LOG=info /usr/bin/time -l cargo run "DarmakzetrekurukzetresamavetAyuyutsavas;mAmakAHpARqavAScEvakimakurvatasaMjaya"
