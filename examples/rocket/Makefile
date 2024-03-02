#Ensure the script is run as bash
SHELL:=/bin/bash

#Set help as the default for this makefile.
.DEFAULT: help

help:
	@echo "Please use 'make <target>' where <target> is one of:"
	@echo ""
	@echo "install                  Install the crate and all required core dependencies"
	@echo "run                      Running the server locally"
	@echo "clean                    Remove all build artifacts"

generate_dot_env:
	@if [[ ! -e .env ]]; then \
		cp .env.example .env; \
	fi

clean:
	rm -fr build/
	rm -fr dist/
	rm -fr target/

install: generate_dot_env
	@echo ""
	@echo "*** Generating a .env file and installing the required dependencies... ***"
	@echo ""
	@echo ""
	@cargo build --release
	@echo ""

run:
	@echo ""
	@echo "*** Running the server locally... ***"
	@echo ""
	@echo ""
	@cargo run --release
	@echo ""
