import pytest

def sum_of_squares(n):
    return sum(x*x for x in range(1, n+1))

@pytest.mark.benchmark(group="Python")
def test_sum_of_squares(benchmark):
    result = benchmark(sum_of_squares, 10_000_000)