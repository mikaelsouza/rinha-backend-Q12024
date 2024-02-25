.PHONY: db
run: 
	docker compose down && docker compose up --build