1️-> Indexer service

Listens to Governor + Timelock events

Saves normalized data

Tech

alloy-rs or ethers-rs

tokio

serde

sqlx


2️-> Database (simple)

Start with:

PostgreSQL (recommended)

Tables:

proposals

votes

delegations

executions


3️-> API service

Reads from DB

Serves frontend

Tech

axum (clean & modern)

REST (simpler than GraphQL at first)



4️-> Frontend (test only)

React

Just:

proposal list

proposal detail

vote button (direct chain call)




Base RPC
  ↓
Rust indexer (events)
  ↓
Postgres
  ↓
Rust API
  ↓
React (read-only)
