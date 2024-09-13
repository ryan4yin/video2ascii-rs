default:
  @just --list

# Auto-format the source tree
fmt:
  treefmt

# Build and run the project via Nix & Crane
run:
  nix run .#default

# Build the project via Nix & Crane
build:
  nix build .#default

# Run 'cargo run' on the project
drun *ARGS:
  nix develop --command cargo run {{ARGS}}

dbuild:
  nix develop --command cargo build

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
  cargo watch -x "run -- {{ARGS}}"
