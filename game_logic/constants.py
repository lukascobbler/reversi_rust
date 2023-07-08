import copy

_WHITE = -1
_BLACK = 1

PLAYER = _BLACK
BOT = _WHITE
EMPTY = 0


def set_player_black():
    global PLAYER, BOT
    PLAYER, BOT = _BLACK, _WHITE


def set_player_white():
    global PLAYER, BOT
    PLAYER, BOT = _WHITE, _BLACK


def generate_starting_board():
    empty_row = [EMPTY] * 8
    black_white_row = [EMPTY] * 3 + [_BLACK, _WHITE] + [EMPTY] * 3
    white_black_row = [EMPTY] * 3 + [_WHITE, _BLACK] + [EMPTY] * 3

    table = []

    for _ in range(3):
        table.append(copy.deepcopy(empty_row))

    table.append(copy.deepcopy(black_white_row))
    table.append(copy.deepcopy(white_black_row))

    for _ in range(3):
        table.append(copy.deepcopy(empty_row))

    return table
