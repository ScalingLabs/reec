.PHONY: build lint test clean run-image build-image download-test-vectors clean-vectors \
	setup-hive test-pattern-default run-hive run-hive-debug clean-hive-logs

help: ## 📚 Show help for each of the Makefile recipes
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

build: ## 🔨 Build the client
	cargo build --workspace

lint: ## 🧹 Linter check
	cargo clippy --all-targets --all-features --workspace --exclude reec-prover -- -D warnings

SPECTEST_VERSION := v3.0.0
SPECTEST_ARTIFACT := tests_$(SPECTEST_VERSION).tar.gz
SPECTEST_VECTORS_DIR := cmd/ef_tests/reec/vectors

CRATE ?= *
test: $(SPECTEST_VECTORS_DIR) ## 🧪 Run each crate's tests
	cargo test -p '$(CRATE)' --workspace --exclude reec-prover --exclude reec-levm --exclude ef_tests-levm -- --skip test_contract_compilation --skip testito

clean: clean-vectors ## 🧹 Remove build artifacts
	cargo clean
	rm -rf hive

STAMP_FILE := .docker_build_stamp
$(STAMP_FILE): $(shell find crates cmd -type f -name '*.rs') Cargo.toml Dockerfile
	docker build -t reec .
	touch $(STAMP_FILE)

build-image: $(STAMP_FILE) ## 🐳 Build the Docker image

run-image: build-image ## 🏃 Run the Docker image
	docker run --rm -p 127.0.0.1:8545:8545 reec --http.addr 0.0.0.0

$(SPECTEST_ARTIFACT):
	rm -f tests_*.tar.gz # Delete older versions
	curl -L -o $(SPECTEST_ARTIFACT) "https://github.com/ethereum/execution-spec-tests/releases/download/$(SPECTEST_VERSION)/fixtures_stable.tar.gz"

$(SPECTEST_VECTORS_DIR): $(SPECTEST_ARTIFACT)
	mkdir -p $(SPECTEST_VECTORS_DIR) tmp
	tar -xzf $(SPECTEST_ARTIFACT) -C tmp
	mv tmp/fixtures/blockchain_tests/* $(SPECTEST_VECTORS_DIR)

download-test-vectors: $(SPECTEST_VECTORS_DIR) ## 📥 Download test vectors

clean-vectors: ## 🗑️  Clean test vectors
	rm -rf $(SPECTEST_VECTORS_DIR)

ETHEREUM_PACKAGE_REVISION := e13a5b177517dfaac313876df58e6dfdb25b8261
# Shallow clones can't specify a single revision, but at least we avoid working
# the whole history by making it shallow since a given date (one day before our
# target revision).
ethereum-package:
	git clone --single-branch --branch reec-integration https://github.com/owanikin/ethereum-package

checkout-ethereum-package: ethereum-package ## 📦 Checkout specific Ethereum package revision
	cd ethereum-package && \
		git fetch && \
		git checkout $(ETHEREUM_PACKAGE_REVISION)

ENCLAVE ?= reecnet

localnet: stop-localnet-silent build-image checkout-ethereum-package ## 🌐 Start local network
	kurtosis run --enclave $(ENCLAVE) ethereum-package --args-file test_data/network_params.yaml
	docker logs -f $$(docker ps -q --filter ancestor=reec)

localnet-assertoor-blob: stop-localnet-silent build-image checkout-ethereum-package ## 🌐 Start local network with assertoor test
	kurtosis run --enclave $(ENCLAVE) ethereum-package --args-file .github/config/assertoor/network_params_blob.yaml
	docker logs -f $$(docker ps -q --filter ancestor=reec)


localnet-assertoor-tx: stop-localnet-silent build-image checkout-ethereum-package ## 🌐 Start local network with assertoor test
	kurtosis run --enclave $(ENCLAVE) ethereum-package --args-file .github/config/assertoor/network_params_tx.yaml
	docker logs -f $$(docker ps -q --filter ancestor=reec)

stop-localnet: ## 🛑 Stop local network
	kurtosis enclave stop $(ENCLAVE)
	kurtosis enclave rm $(ENCLAVE) --force

stop-localnet-silent:
	@echo "Double checking local net is not already started..."
	@kurtosis enclave stop $(ENCLAVE) >/dev/null 2>&1 || true
	@kurtosis enclave rm $(ENCLAVE) --force >/dev/null 2>&1 || true
