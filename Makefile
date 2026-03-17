.DEFAULT_GOAL := help

.PHONY: devcli help 

devcli: ## Enter the devcontainer shell
	docker exec -it --user vscode --workdir /workspaces/openvault openvault-devtainer /bin/zsh

dev-desktop: ## Run the desktop application in development
	bun run dev --filter @openvault/desktop

help: ## Display this help screen
	@echo
	@echo "\033[1mUsage:\033[0m make <target>"
	@echo
	@echo "\033[1mAvailable Targets:\033[0m"
	@echo

	@grep -E '^[a-zA-Z0-9_-]+:.*?##' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

	@echo
