set windows-shell := ["nu.exe", "-c"]
set shell := ["nu", "-c"]

root := absolute_path('')

default:
    @just --choose

dev *args:
    dx serve --platform desktop

css *args:
    tailwindcss -i '{{ root }}/input.css' -o '{{ root }}/assets/tailwind.css' --watch

test:
    cd '{{ root }}'; cargo test

format:
    cd '{{ root }}'; just --unstable --fmt
    prettier --write '{{ root }}'
    nixpkgs-fmt '{{ root }}'
    cd '{{ root }}'; cargo fmt --all
    cd '{{ root }}'; cargo clippy --fix --allow-dirty

lint:
    cd '{{ root }}'; just --unstable --fmt --check
    prettier --check '{{ root }}'
    cspell lint '{{ root }}' --no-progress
    nixpkgs-fmt '{{ root }}' --check
    markdownlint --ignore-path .gitignore '{{ root }}'
    markdown-link-check \
      --config .markdown-link-check.json \
      --quiet \
      ...(fd '.*.md' | lines)
    cd '{{ root }}'; cargo clippy -- -D warnings

upgrade:
    nix flake update
    cargo upgrade
