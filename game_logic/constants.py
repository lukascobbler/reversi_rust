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
    prazan_red = [EMPTY] * 8
    crno_beli_red = [EMPTY] * 3 + [_BLACK, _WHITE] + [EMPTY] * 3
    belo_crni_red = [EMPTY] * 3 + [_WHITE, _BLACK] + [EMPTY] * 3

    tabla = []

    for velicina in range(3):
        tabla.append(copy.deepcopy(prazan_red))

    tabla.append(copy.deepcopy(crno_beli_red))
    tabla.append(copy.deepcopy(belo_crni_red))

    for velicina in range(3):
        tabla.append(copy.deepcopy(prazan_red))

    return tabla
