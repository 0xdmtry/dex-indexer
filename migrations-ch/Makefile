.PHONY: help up status create reset test-connection init-table

# Colors
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
NC := \033[0m # No Color

# Load environment variables from .env if it exists
ifneq (,$(wildcard .env))
    include .env
    export
endif

# Configuration with defaults
CLICKHOUSE_HOST ?= localhost
CLICKHOUSE_PORT ?= 9000
CLICKHOUSE_USER ?= clickhouse_user
CLICKHOUSE_PASSWORD ?= clickhouse_password
CLICKHOUSE_DB ?= events_db
CLICKHOUSE_CONTAINER ?= clickhouse

MIGRATIONS_DIR := sql
MIGRATIONS_TABLE := schema_migrations

# Detect if we should use Docker or local client
USE_DOCKER ?= true

# ClickHouse client command
ifeq ($(USE_DOCKER),true)
    # Use Docker to run clickhouse-client
    CH_CLIENT := docker exec -i $(CLICKHOUSE_CONTAINER) clickhouse-client \
        --user=$(CLICKHOUSE_USER) \
        --password=$(CLICKHOUSE_PASSWORD) \
        --database=$(CLICKHOUSE_DB)

    # For file execution, we need to cat the file and pipe it
    CH_CLIENT_FILE = docker exec -i $(CLICKHOUSE_CONTAINER) clickhouse-client \
        --user=$(CLICKHOUSE_USER) \
        --password=$(CLICKHOUSE_PASSWORD) \
        --database=$(CLICKHOUSE_DB) \
        --multiquery <
else
    # Use local clickhouse-client (requires installation)
    CH_CLIENT := clickhouse-client \
        --host=$(CLICKHOUSE_HOST) \
        --port=$(CLICKHOUSE_PORT) \
        --user=$(CLICKHOUSE_USER) \
        --password=$(CLICKHOUSE_PASSWORD) \
        --database=$(CLICKHOUSE_DB)

    CH_CLIENT_FILE = $(CH_CLIENT) --queries-file=
endif

# Default target
help:
	@echo "$(YELLOW)ClickHouse Migration Tool$(NC)"
	@echo ""
	@echo "Usage: make <command>"
	@echo ""
	@echo "Commands:"
	@echo "  $(GREEN)up$(NC)              Run all pending migrations"
	@echo "  $(GREEN)status$(NC)          Show migration status"
	@echo "  $(GREEN)create$(NC)          Create a new migration file (use name=<migration_name>)"
	@echo "  $(GREEN)reset$(NC)           Reset database (drops all tables) - DANGEROUS!"
	@echo "  $(GREEN)test-connection$(NC) Test ClickHouse connection"
	@echo "  $(GREEN)info$(NC)            Show current configuration"
	@echo "  $(GREEN)help$(NC)            Show this help message"
	@echo ""
	@echo "Environment Variables:"
	@echo "  CLICKHOUSE_CONTAINER ClickHouse container name (default: clickhouse)"
	@echo "  CLICKHOUSE_USER      ClickHouse user (default: clickhouse_user)"
	@echo "  CLICKHOUSE_PASSWORD  ClickHouse password"
	@echo "  CLICKHOUSE_DB        ClickHouse database (default: events_db)"
	@echo "  USE_DOCKER           Use Docker client (default: true)"
	@echo ""
	@echo "Examples:"
	@echo "  make up"
	@echo "  make status"
	@echo "  make create name=add_user_table"
	@echo "  USE_DOCKER=false CLICKHOUSE_HOST=prod-host make up"
	@echo ""

# Test connection to ClickHouse
test-connection:
	@echo "$(YELLOW)Testing ClickHouse connection...$(NC)"
	@if $(CH_CLIENT) --query "SELECT 1" > /dev/null 2>&1; then \
		echo "$(GREEN)✓ Connection successful$(NC)"; \
		if [ "$(USE_DOCKER)" = "true" ]; then \
			echo "Container: $(CLICKHOUSE_CONTAINER)"; \
		else \
			echo "Host: $(CLICKHOUSE_HOST):$(CLICKHOUSE_PORT)"; \
		fi; \
		echo "Database: $(CLICKHOUSE_DB)"; \
	else \
		echo "$(RED)✗ Connection failed$(NC)"; \
		if [ "$(USE_DOCKER)" = "true" ]; then \
			echo "Container: $(CLICKHOUSE_CONTAINER)"; \
			echo "Make sure the container is running: docker ps | grep $(CLICKHOUSE_CONTAINER)"; \
		else \
			echo "Host: $(CLICKHOUSE_HOST):$(CLICKHOUSE_PORT)"; \
			echo "User: $(CLICKHOUSE_USER)"; \
		fi; \
		echo "Database: $(CLICKHOUSE_DB)"; \
		exit 1; \
	fi

# Initialize migrations tracking table
init-table: test-connection
	@echo "$(YELLOW)Initializing migrations tracking table...$(NC)"
	@$(CH_CLIENT) --query "\
		CREATE TABLE IF NOT EXISTS $(MIGRATIONS_TABLE) (\
			version UInt32,\
			name String,\
			applied_at DateTime DEFAULT now()\
		) ENGINE = MergeTree()\
		ORDER BY version\
	" > /dev/null 2>&1
	@echo "$(GREEN)✓ Migrations table ready$(NC)"

# Run all pending migrations
up: init-table
	@echo "$(YELLOW)Running migrations...$(NC)"
	@applied=0; \
	skipped=0; \
	for migration in $$(ls $(MIGRATIONS_DIR)/*.sql 2>/dev/null | sort); do \
		filename=$$(basename $$migration); \
		version=$$(echo $$filename | grep -oE '^[0-9]+'); \
		name=$$(echo $$filename | sed 's/^[0-9]*_//' | sed 's/.sql$$//'); \
		echo "$(YELLOW)Processing: $$filename$(NC)"; \
		count=$$($(CH_CLIENT) --query "SELECT count() FROM $(MIGRATIONS_TABLE) WHERE version = $$version" 2>/dev/null || echo "0"); \
		if [ "$$count" -gt 0 ]; then \
			echo "$(YELLOW)⊘ Skipping $$filename (already applied)$(NC)"; \
			skipped=$$((skipped + 1)); \
		else \
			echo "$(GREEN)→ Applying $$filename$(NC)"; \
			sql_content=$$(cat $$migration); \
			if docker exec $(CLICKHOUSE_CONTAINER) clickhouse-client \
				--user=$(CLICKHOUSE_USER) \
				--password=$(CLICKHOUSE_PASSWORD) \
				--database=$(CLICKHOUSE_DB) \
				--query="$$sql_content" > /dev/null 2>&1; then \
				$(CH_CLIENT) --query "INSERT INTO $(MIGRATIONS_TABLE) (version, name) VALUES ($$version, '$$name')" > /dev/null 2>&1; \
				echo "$(GREEN)✓ Applied $$filename$(NC)"; \
				applied=$$((applied + 1)); \
			else \
				echo "$(RED)✗ Failed to apply $$filename$(NC)"; \
				docker exec $(CLICKHOUSE_CONTAINER) clickhouse-client \
					--user=$(CLICKHOUSE_USER) \
					--password=$(CLICKHOUSE_PASSWORD) \
					--database=$(CLICKHOUSE_DB) \
					--query="$$sql_content"; \
				exit 1; \
			fi; \
		fi; \
	done; \
	echo ""; \
	echo "$(GREEN)Migration summary:$(NC)"; \
	echo "  Applied: $$applied"; \
	echo "  Skipped: $$skipped"; \
	echo ""

# Show migration status
status: init-table
	@echo "$(YELLOW)Migration Status$(NC)"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@current_version=$$($(CH_CLIENT) --query "SELECT max(version) FROM $(MIGRATIONS_TABLE)" 2>/dev/null || echo "0"); \
	echo "Current version: $$current_version"; \
	echo ""; \
	echo "Applied migrations:"; \
	$(CH_CLIENT) --query "SELECT version, name, applied_at FROM $(MIGRATIONS_TABLE) ORDER BY version FORMAT Pretty" 2>/dev/null || echo "None"; \
	echo ""; \
	echo "Available migrations:"; \
	for migration in $$(ls $(MIGRATIONS_DIR)/*.sql 2>/dev/null | sort); do \
		filename=$$(basename $$migration); \
		version=$$(echo $$filename | grep -oE '^[0-9]+'); \
		count=$$($(CH_CLIENT) --query "SELECT count() FROM $(MIGRATIONS_TABLE) WHERE version = $$version" 2>/dev/null || echo "0"); \
		if [ "$$count" -gt 0 ]; then \
			echo "  $(GREEN)✓$(NC) $$filename"; \
		else \
			echo "  $(YELLOW)⊘$(NC) $$filename (pending)"; \
		fi; \
	done; \
	echo ""

# Create a new migration file
create:
ifndef name
	@echo "$(RED)Error: Migration name required$(NC)"
	@echo "Usage: make create name=<migration_name>"
	@exit 1
endif
	@last_version=0; \
	for file in $(MIGRATIONS_DIR)/*.sql 2>/dev/null; do \
		if [ -f "$$file" ]; then \
			version=$$(basename "$$file" | grep -oE '^[0-9]+'); \
			if [ "$$version" -gt "$$last_version" ]; then \
				last_version=$$version; \
			fi; \
		fi; \
	done; \
	next_version=$$(printf "%03d" $$((last_version + 1))); \
	filename="$${next_version}_$(name).sql"; \
	filepath="$(MIGRATIONS_DIR)/$$filename"; \
	mkdir -p $(MIGRATIONS_DIR); \
	echo "-- Migration: $(name)" > $$filepath; \
	echo "-- Version: $$next_version" >> $$filepath; \
	echo "-- Created: $$(date)" >> $$filepath; \
	echo "" >> $$filepath; \
	echo "-- Your SQL here" >> $$filepath; \
	echo "" >> $$filepath; \
	echo "$(GREEN)✓ Created migration: $$filename$(NC)"; \
	echo "Edit: $$filepath"

# Reset database (dangerous!)
reset: test-connection
	@echo "$(RED)⚠️  WARNING: This will drop all tables and reset migrations!$(NC)"
	@read -p "Are you sure? Type 'yes' to confirm: " confirm; \
	if [ "$$confirm" != "yes" ]; then \
		echo "Aborted."; \
		exit 0; \
	fi; \
	echo "$(YELLOW)Resetting database...$(NC)"; \
	tables=$$($(CH_CLIENT) --query "SHOW TABLES" | grep -v "SHOW TABLES"); \
	for table in $$tables; do \
		echo "Dropping table: $$table"; \
		$(CH_CLIENT) --query "DROP TABLE IF EXISTS $$table" > /dev/null; \
	done; \
	echo "$(GREEN)✓ Database reset complete$(NC)"

# Info target to show current configuration
info:
	@echo "$(YELLOW)Current Configuration:$(NC)"
	@if [ "$(USE_DOCKER)" = "true" ]; then \
		echo "  Mode:       Docker"; \
		echo "  Container:  $(CLICKHOUSE_CONTAINER)"; \
	else \
		echo "  Mode:       Local Client"; \
		echo "  Host:       $(CLICKHOUSE_HOST)"; \
		echo "  Port:       $(CLICKHOUSE_PORT)"; \
	fi
	@echo "  User:       $(CLICKHOUSE_USER)"
	@echo "  Database:   $(CLICKHOUSE_DB)"
	@echo ""