.DEFAULT_GOAL := help

L2_GENESIS_FILE_PATH=../../test_data/genesis-l2.json
L1_GENESIS_FILE_PATH=../../test_data/genesis-l1.json

# Basic
.PHONY: help init down clean restart

help: ## 📚 Show help for each of the Makefile recipes
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

init: init-local-l1 deploy-l1 init-l2 ## 🚀 Initializes a localnet with reec client as both L1 and L2

down: down-local-l1 down-l2 ## 🛑 Shuts down the localnet

clean: clean-contract-deps ## 🧹 Cleans the localnet

restart: restart-local-l1 deploy-l1 restart-l2 ## 🔄 Restarts the localnet

# CLI
.PHONY: cli update-cli-contracts

cli: ## 🛠️ Installs the L2 reec CLI
	cargo install --path ${reec_PATH}/cmd/reec_l2/ --force

update-cli-contracts: ## 📜 Update the CLI's config contracts
	@if [ -z "$$C" ]; then \
		echo "Error: CONFIG_NAME (C) is missing.\nPlease provide it as an argument:\nmake update-cli-contracts C=<config_name>."; \
		exit 1; \
	fi && \
	CB=$$(grep 'L1_WATCHER_BRIDGE_ADDRESS' .env | cut -d= -f2) && \
	reec_l2 config edit --common-bridge $$CB $$C && \
	OP=$$(grep 'COMMITTER_ON_CHAIN_PROPOSER_ADDRESS' .env | cut -d= -f2) && \
	reec_l2 config edit --on-chain-proposer $$OP $$C

# Variables

reec_PATH=$(shell pwd)/../..
reec_BIN_PATH=$(reec_PATH)/target/release/reec
reec_DEV_DOCKER_COMPOSE_PATH=$(reec_PATH)/crates/blockchain/dev/docker-compose-dev.yaml
reec_L2_DOCKER_COMPOSE_PATH=./docker-compose-l2.yaml

reec_L2_CONTRACTS_PATH=./contracts
L1_RPC_URL=http://localhost:8545
L1_PRIVATE_KEY=0x385c546456b6a603a1cfcaa9ec9494ba4832da08dd6bcf4de9a71e4a01b74924

reec_L2_DEV_LIBMDBX=dev_reec_l2
reec_L1_DEV_LIBMDBX=dev_reec_l1
L1_PORT=8545
L2_PORT=1729
L1_AUTH_PORT=8551
# Used in the .env file. Ensure the same port is used for `ENGINE_API_RPC_URL`.
L2_AUTH_PORT=8552

# Local L1
.PHONY: init-local-l1 init-l1 down-local-l1 restart-local-l1 rm-db-l1

init-local-l1: ## 🚀 Initializes an L1 reec Client with Docker (Used with make init)
	docker compose -f ${reec_DEV_DOCKER_COMPOSE_PATH} up -d

init-l1: ## 🚀 Initializes an L1 reec Client
	cargo run --release --manifest-path ../../Cargo.toml --bin reec --features dev -- \
	--network ${L1_GENESIS_FILE_PATH} \
	--http.port ${L1_PORT} \
	--http.addr 0.0.0.0 \
	--authrpc.port ${L1_AUTH_PORT} \
	--datadir ${reec_L1_DEV_LIBMDBX}

init-l1-levm: ## 🚀 Initializes an L1 reec Client with LEVM
	cargo run --release \
    --manifest-path ../../Cargo.toml \
    --bin reec \
    --features dev,reec-blockchain/levm,reec-vm/levm -- \
    --network ${L1_GENESIS_FILE_PATH} \
    --http.port ${L1_PORT} \
    --http.addr 0.0.0.0 \
    --authrpc.port ${L1_AUTH_PORT} \
    --datadir ${reec_L1_DEV_LIBMDBX}

down-local-l1: ## 🛑 Shuts down the L1 reec Client
	docker compose -f ${reec_DEV_DOCKER_COMPOSE_PATH} down
	docker compose -f docker-compose-l2.yaml down

restart-local-l1: down-local-l1 init-local-l1 ## 🔄 Restarts the L1 reec Client

rm-db-l1: ## 🛑 Removes the DB used by the L1
	cargo run --release --manifest-path ../../Cargo.toml --bin reec -- removedb --datadir ${reec_L1_DEV_LIBMDBX}

# Contracts
PHONY: clean-contract-deps restart-contract-deps deploy-l1

clean-contract-deps: ## 🧹 Cleans the dependencies for the L1 contracts.
	rm -rf contracts/solc_out
	rm -rf contracts/lib

restart-contract-deps: clean-contract-deps ## 🔄 Restarts the dependencies for the L1 contracts.

deploy-l1: ## 📜 Deploys the L1 contracts
	DEPLOYER_CONTRACTS_PATH=contracts cargo run --release --bin reec_l2_l1_deployer --manifest-path ${reec_L2_CONTRACTS_PATH}/Cargo.toml

# L2
PHONY: init-l2 down-l2 restart-l2 init-prover rm-db-l2

init-l2: ## 🚀 Initializes an L2 reec Client
	cargo run --release --manifest-path ../../Cargo.toml --bin reec --features l2 -- \
	--network ${L2_GENESIS_FILE_PATH} \
	--http.port ${L2_PORT} \
	--http.addr 0.0.0.0 \
	--authrpc.port ${L2_AUTH_PORT} \
	--datadir ${reec_L2_DEV_LIBMDBX}

down-l2: ## 🛑 Shuts down the L2 reec Client
	pkill -f reec || exit 0

restart-l2: down-l2 init-l2 ## 🔄 Restarts the L2 reec Client

init-prover: ## 🚀 Initializes the Prover
	@if [ -z "$$T" ]; then \
		echo "Error: ProverType (T) is missing."; \
		echo "Please provide it as an argument:"; \
		echo "make init-prover T=<prover_type: (risc0, sp1)> <G=true>."; \
		echo "The prover can also be run with GPU (G)"; \
		exit 1; \
	fi; \

	@if [ -z "$$G" ]; then \
		GPU=""; \
	else \
		GPU=",gpu"; \
	fi; \
	cargo run --release --features "build_$$T$$GPU" --manifest-path ../../Cargo.toml --bin reec_prover -- $$T

rm-db-l2: ## 🛑 Removes the DB used by the L2
	cargo run --release --manifest-path ../../Cargo.toml --bin reec -- removedb --datadir ${reec_L2_DEV_LIBMDBX}

# Testing
PHONY: ci_test test

ci_test: ## 🚧 Runs the L2's integration test, used by the github's CI
	docker compose -f ${reec_L2_DOCKER_COMPOSE_PATH} down
	docker compose -f ${reec_L2_DOCKER_COMPOSE_PATH} up -d --build
	BRIDGE_ADDRESS=$$(grep 'L1_WATCHER_BRIDGE_ADDRESS' .env | cut -d= -f2) ON_CHAIN_PROPOSER_ADDRESS=$$(grep 'COMMITTER_ON_CHAIN_PROPOSER_ADDRESS' .env | cut -d= -f2) cargo test --release testito -- --nocapture
	docker compose -f ${reec_L2_DOCKER_COMPOSE_PATH} down

test: ## 🚧 Runs the L2's integration test, run `make init` and in a new terminal make test
	BRIDGE_ADDRESS=$$(grep 'L1_WATCHER_BRIDGE_ADDRESS' .env | cut -d= -f2) ON_CHAIN_PROPOSER_ADDRESS=$$(grep 'COMMITTER_ON_CHAIN_PROPOSER_ADDRESS' .env | cut -d= -f2) cargo test --release testito -- --nocapture
