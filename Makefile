.PHONY: run


run :
	( cd riddles && echo "" > logs/riddles.log && cargo fmt && cargo run --bin riddle-34 	)