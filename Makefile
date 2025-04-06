.PHONY: help rub.web

# Default target
.DEFAULT_GOAL := help

# Colors for help output
BLUE := \033[34m
GREEN := \033[32m
RESET := \033[0m

help: ## Display this help message
	@echo "$(BLUE)Available targets:$(RESET)"
	@grep -E '^[a-zA-Z0-9_.-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(RESET) %s\n", $$1, $$2}'

run.web: ## Run the client app in web browser via WASM
	cd client && cargo run --target wasm32-unknown-unknown

run.native: ## Run the client as a native desktop app
	cd client && cargo run