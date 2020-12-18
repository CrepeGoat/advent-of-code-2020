import pytest

from AoC2020_python import day15part1


@pytest.mark.parametrize('starters, expt_sequence', [
    ([0, 3, 6], [0, 3, 6, 0, 3, 3, 1, 0, 4, 0]),
])
def test_play_the_game(starters, expt_sequence):
    for calc, expt in zip(day15part1.play_the_game(*starters), expt_sequence):
        assert calc == expt


@pytest.mark.parametrize('starters, expt_2020_result', [
    ([1, 3, 2], 1),
    ([2, 1, 3], 10),
    ([1, 2, 3], 27),
    ([2, 3, 1], 78),
    ([3, 2, 1], 438),
    ([3, 1, 2], 1836),
    ([1, 12, 0, 20, 8, 16], 273)
])
def test_get_2020th(starters, expt_2020_result):
    assert day15part1.get_2020th(*starters) == expt_2020_result
