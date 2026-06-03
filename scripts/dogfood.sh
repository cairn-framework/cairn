#!/bin/sh
# Dogfood gate: run cairn against itself.
# Called from pre-push hook and CI workflow.
set -e

echo "== cairn lint =="
cairn lint

echo "== cairn hook all =="
cairn hook all

echo "== dogfood pass =="
