# Rust Python Template

## develop
```shell
python3 -m venv .venv
source .venv/bin/activate
pip install maturin
maturin develop
# or
maturin build
```

## test

```shell
pip install pytest

cd tests/
# select
pytest -v -m fib
# deselect
pytest -v -m "not fib"
```