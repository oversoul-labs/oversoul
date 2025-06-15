#!/bin/bash

echo "▶ Setting up virtual environment..."
python3 -m venv .venv
source .venv/bin/activate
uv pip install -r <(uv pip compile pyproject.toml)
