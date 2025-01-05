# Trains a padaccheda model and saves important features to disk.
# NOTE: when training, exclude the file paths used in `make eval`.
create-cheda-model:
	cargo run --release --bin create_cheda_model -- \
		--model-dir "../data/build/vidyut-latest" \
		--include "../data/raw/dcs/conllu/files/**/*.conllu" \
		--exclude "../data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--exclude "../data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--exclude "../data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"

# Evaluate our parsing quality on a large sample of text.
eval_cheda:
	cargo run --release --bin eval_cheda -- \
		--vidyut-dir "data/build/vidyut-latest" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-088*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-089*.conllu" \
		--paths "data/raw/dcs/conllu/files/Mahābhārata/Mahābhārata-0900-MBh, 6, BhaGī 18-7707.conllu"



# Creates and collects all data files required to use Vidyut.
create_all_data:
	@./scripts/create_all_data.sh

create_sandhi_rules:
	RUST_LOG=info cargo run --release --bin create_sandhi_rules -- \
			 --data-dir data/build/vidyut-latest

# Creates a kosha and write it to disk.
create_kosha:
	RUST_LOG=info cargo run --release --bin create_kosha -- \
			 --input-dir data/raw/lex \
			 --dhatupatha ../vidyut-prakriya/data/dhatupatha.tsv \
			 --output-dir data/build/vidyut-latest/kosha

# Integration tests
# =================

# Runs basic end-to-end tests against the given kosha.
test_kosha:
	RUST_LOG=info cargo run --release --bin test_kosha -- \
		--data-dir data/build/vidyut-latest/kosha

