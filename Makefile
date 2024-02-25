.PHONY: db
db: 
	docker compose down && docker compose up --build