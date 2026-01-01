.PHONY: install-all uninstall-all status help

help:
	@echo "Available targets:"
	@echo "  install-all        - Install everything (namespaces, operators, infrastructure, ui-tools, microservices, observability)"
	@echo "  uninstall-all      - Uninstall everything"
	@echo "  status             - Show cluster status"
	@echo ""
	@echo "Individual steps:"
	@echo "  make -f 100-Makefile-namespaces install-namespaces"
	@echo "  make -f 200-Makefile-operators install-operators"
	@echo "  make -f 300-Makefile-infrastructure install-infrastructure"
	@echo "  make -f 400-Makefile-ui-tools install-ui-tools"
	@echo "  make -f 500-Makefile-microservices build-images"
	@echo "  make -f 500-Makefile-microservices install-microservices"
	@echo "  make -f 600-Makefile-observability install-observability"

install-all:
	@echo "Starting full installation..."
	@make -f 100-Makefile-namespaces install-namespaces
	@make -f 200-Makefile-operators install-operators
	@make -f 300-Makefile-infrastructure install-infrastructure
	@make -f 400-Makefile-ui-tools install-ui-tools
	@make -f 500-Makefile-microservices build-images
	@make -f 500-Makefile-microservices install-microservices
	@make -f 600-Makefile-observability install-observability
	@echo ""
	@echo "=========================================="
	@echo "Installation complete!"
	@echo "=========================================="
	@echo ""
	@echo "Access endpoints:"
	@echo "  pump-api:           http://localhost:30800"
	@echo "  pump-ws-streamer:   http://localhost:30805"
	@echo "  pgAdmin:            http://localhost:30082"
	@echo "  Kafka UI:           http://localhost:30080"
	@echo "  Redis Commander:    http://localhost:30081"
	@echo "  ClickHouse UI:      http://localhost:30083"
	@echo "  ClickHouse (HTTP):  http://localhost:31823"
	@echo "  ClickHouse (TCP):   localhost:31900"
	@echo "  Prometheus:         http://localhost:30090"
	@echo "  Grafana:            http://localhost:30300"
	@echo ""

uninstall-all:
	@echo "Starting full uninstallation..."
	@make -f 600-Makefile-observability uninstall-observability
	@make -f 500-Makefile-microservices uninstall-microservices
	@make -f 400-Makefile-ui-tools uninstall-ui-tools
	@make -f 300-Makefile-infrastructure uninstall-infrastructure
	@make -f 200-Makefile-operators uninstall-operators
	@make -f 100-Makefile-namespaces uninstall-namespaces
	@echo "Uninstallation complete."

status:
	@echo "Cluster Status"
	@echo "=============="
	@echo ""
	@echo "Namespaces:"
	@kubectl get namespaces | grep -E 'infrastructure|microservices|observability|ui-tools|kafka|minio|cnpg-system' || true
	@echo ""
	@echo "Infrastructure (infrastructure namespace):"
	@kubectl get pods -n infrastructure
	@echo ""
	@echo "Kafka (kafka namespace):"
	@kubectl get pods -n kafka
	@echo ""
	@echo "Microservices (microservices namespace):"
	@kubectl get pods -n microservices
	@echo ""
	@echo "UI Tools (ui-tools namespace):"
	@kubectl get pods -n ui-tools
	@echo ""
	@echo "Observability (observability namespace):"
	@kubectl get pods -n observability