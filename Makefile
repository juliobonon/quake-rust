help: # Generate a list of available commands and their descriptions
	@echo "Available commands:"
	@awk 'BEGIN {FS = ":.*?# "} /^[a-zA-Z0-9_-]+:.*?# / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

setup: # Setup the project
	@echo "Setting up the project..."
	@cp qgames.log $(HOME)/qgames.log
	@echo "QGames log moved to $(HOME)/qgames.log"
	@echo "Project setup successfully!"

docs: # Generate the project documentation
	@echo "Generating the project documentation..."
	@cargo doc --no-deps --open
	@echo "Documentation generated successfully!"

run-docker: # Start the project on a docker container
	@echo "Starting the project..."
	@docker-compose run --rm parser
	@echo "Project started successfully!"

build-docker: # Build the project using multi-stage builds
	@echo "Building the project..."
	@docker-compose build --pull
	@echo "Project built successfully!"

test: # Run the tests
	@echo "Running the tests..."
	@cargo test
	@echo "Tests ran successfully!"

build: # Build the project
	@echo "Building the project..."
	@cargo build --release
	@echo "Project built successfully!"

run: # Run the project
	@echo "Running the project..."
	@cargo run --release
	@echo "Project ran successfully!"
