import rpy
import pytest

@pytest.mark.fib
def test_rpy_fib():
  for i in (0, 20):
    assert rpy.fib(i) == rpy.fib_py(i)