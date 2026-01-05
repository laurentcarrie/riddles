.PHONY: run


run :
	( cd riddles && cargo fmt && cargo clippy && cargo build && echo "" > logs/riddles.log && cargo fmt && cargo run --bin riddle-34 	)