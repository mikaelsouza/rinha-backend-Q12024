.PHONY: run
run: 
	docker compose down && cargo build --release && docker compose up --build