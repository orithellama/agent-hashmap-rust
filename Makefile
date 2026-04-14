.PHONY: help init bootstrap index reindex clean \
get set del search list stats doctor export import \
test fmt lint release demo

help:
	@echo "AgentMem Commands:"
	@echo "  make init"
	@echo "  make bootstrap"
	@echo "  make index"
	@echo "  make reindex"
	@echo "  make get KEY=repo/stack"
	@echo "  make set KEY=task VALUE='Fix auth'"
	@echo "  make del KEY=task"
	@echo "  make search QUERY=auth"
	@echo "  make list"
	@echo "  make stats"
	@echo "  make doctor"
	@echo "  make export"
	@echo "  make import FILE=backup.json"

init:
	agentmem init

bootstrap:
	agentmem init --yes
	agentmem index

index:
	agentmem index

reindex:
	agentmem reindex

clean:
	rm -rf .agentmem/index*

get:
	@if [ -z "$(KEY)" ]; then echo "Usage: make get KEY=repo/stack"; exit 1; fi
	agentmem get "$(KEY)"

set:
	@if [ -z "$(KEY)" ]; then echo "Usage: make set KEY=task VALUE='Fix auth'"; exit 1; fi
	agentmem set "$(KEY)" "$(VALUE)"

del:
	@if [ -z "$(KEY)" ]; then echo "Usage: make del KEY=task"; exit 1; fi
	agentmem delete "$(KEY)"

search:
	@if [ -z "$(QUERY)" ]; then echo "Usage: make search QUERY=login"; exit 1; fi
	agentmem search "$(QUERY)"

list:
	agentmem list

stats:
	agentmem stats

doctor:
	agentmem doctor

export:
	agentmem export > agentmem-backup.json

import:
	@if [ -z "$(FILE)" ]; then echo "Usage: make import FILE=backup.json"; exit 1; fi
	agentmem import "$(FILE)"

test:
	cargo test --workspace --all-features

fmt:
	cargo fmt --all

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

release:
	cargo build --release

demo:
	agentmem set repo/name "$(shell basename $$PWD)"
	agentmem set repo/path "$(PWD)"
	agentmem index