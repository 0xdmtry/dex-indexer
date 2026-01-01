# Solana Indexer — Streaming & Processing Platform

![Architecture](assets/arch-v018.png)

A **modular Solana indexing platform** for **streaming, processing, enriching, and persisting on-chain data from Solana
DEXs** (e.g. Pump.fun, PumpSwap, Raydium, Meteora).
Designed as a **layered, protocol-agnostic system** suitable for both real-time and historical workloads.

---

## System Overview

The platform is composed of isolated services grouped by responsibility, connected through a shared network and an
event-driven backbone.

**Core goals**

* High-throughput transaction ingestion
* Deterministic event processing
* Separation of OLTP, OLAP, cache, and archival storage
* Replayable and auditable pipelines
* Protocol-agnostic extensibility

---

## Architectural Layers

### 1. Extraction Layer

Responsible for ingesting on-chain data from Solana.

**Components**

* **Geyser Transaction Streamer**
  Streams Solana transactions, detects protocol-specific instructions, and emits structured events.
* **Geyser Account Subscriber**
  Subscribes to account state changes (bonding curves, pools, vaults).
* **Gap Filler**
  Detects missing slots / signatures and performs corrective backfills.
* **Historian**
  Executes scheduled historical backfills to ensure long-term completeness.

**Output**: normalized events published to Kafka.

---

### 2. Transport Layer

Provides decoupled, durable, and scalable event delivery.

**Components**

* **Kafka Broker**
  Central event bus for all ingestion and processing flows.
* **Kafka UI**
  Operational inspection of topics, partitions, and consumer groups.

**Typical Topics**

* Trade events (per protocol)
* Token creation and migration events
* Price and fulfillment requests
* Backfill and reconciliation signals

---

### 3. Processing & Preservation Layer

Consumes events, enriches them, and persists results.

**Component**

* **Data Processor**

    * Consumes Kafka topics
    * Normalizes protocol-specific data into unified schemas
    * Performs enrichment and validation
    * Routes data to the appropriate storage backend

**Responsibilities**

* Canonical trade and token records
* Analytical event modeling
* Cache updates
* Raw data archiving

---

### 4. Persistence & Caching Layer

Dedicated storage systems, each optimized for a specific workload.

**Relational Storage**

* **PostgreSQL**
  Canonical state: tokens, pools, trades, metadata.

**Analytical Storage**

* **ClickHouse**
  High-volume event data, time-series analytics, aggregations.

**Caching**

* **Redis**
  Hot state, deduplication, ephemeral coordination data.

**Object Storage**

* **MinIO (S3-compatible)**
  Raw transactions, snapshots, historical archives, and large immutable datasets.

**Administrative UIs**

* pgAdmin (PostgreSQL)
* ClickHouse UI
* Redis Commander

---

### 5. Public Layer

Exposes indexed and processed data to external consumers.

**Component**

* **Public API**

    * Serves normalized data
    * Reads from cache and databases
    * Designed for dashboards, bots, and downstream services

---

### 6. Observability Layer

Provides operational visibility and monitoring.

**Components**

* **Prometheus**
  Metrics collection across services.
* **Grafana**
  Dashboards for ingestion, processing, lag, and storage health.

---

## Data Flow Summary

1. On-chain data is streamed from Solana (transactions and accounts).
2. Events are published to Kafka in protocol-specific topics.
3. Processing services consume, normalize, and enrich events.
4. Data is persisted according to access patterns:

    * PostgreSQL → relational state
    * ClickHouse → analytics
    * Redis → hot cache
    * S3 (MinIO) → raw and archival data
5. Public APIs and analytics systems consume processed outputs.
6. Observability services monitor the entire pipeline.

---

## Design Characteristics

* Event-driven architecture (Kafka-centric)
* Clear separation of concerns per layer
* Protocol-agnostic ingestion and processing
* Deterministic, replayable pipelines
* Independent scalability of services
* OLTP / OLAP / cache / archive isolation
* S3-compatible long-term storage

---

## Supported Workloads

* Real-time DEX trade indexing
* Token lifecycle tracking
* Pool and bonding-curve state monitoring
* Historical backfills and reconciliation
* High-volume analytical queries
* Downstream APIs, bots, and dashboards

---

## Intended Deployment Model

* Containerized microservices
* Network-isolated service mesh
* Compatible with orchestrators such as Kubernetes
* Suitable for both continuous streaming and batch workloads
