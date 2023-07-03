import game_logic.constants as constants


def calculate_opposite_player(current_player):
    return current_player * -1


def is_terminal(table, current_player):
    for row in range(8):
        for column in range(8):
            if is_field_legal(table, row, column, current_player):
                return False

    return True


def num_disks(table):
    k = 0
    i = 0
    for row in range(8):
        for column in range(8):
            i += table[row][column] == constants.PLAYER
            k += table[row][column] == constants.BOT

    return k, i


def all_legal_moves(table, current_player):
    moves_list = []

    for row in range(8):
        for column in range(8):
            if is_field_legal(table, row, column, current_player):
                moves_list.append((row, column))

    return moves_list


def in_bounds(x, y):
    return 0 <= x < 8 and 0 <= y < 8


def is_field_legal(table, startx, starty, current_player):
    if table[startx][starty] != constants.EMPTY:
        return False

    opposite_player = calculate_opposite_player(current_player)

    directions = ((1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1))

    for x_dir, y_dir in directions:
        x, y = startx, starty
        counter = 0

        x += x_dir
        y += y_dir

        while in_bounds(x, y) and table[x][y] == opposite_player:
            counter += 1
            x += x_dir
            y += y_dir

        if in_bounds(x, y) and table[x][y] == current_player and counter > 0:
            return True

    return False


def next_table(table, startx, starty, current_player):
    opposite_player = calculate_opposite_player(current_player)

    directions = ((1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1))

    for x_dir, y_dir in directions:
        x, y = startx, starty
        x += x_dir
        y += y_dir

        flips = []

        while in_bounds(x, y):
            if table[x][y] == opposite_player:
                flips.append((x, y))
            elif table[x][y] == current_player:
                for x_flip, y_flip in flips:
                    table[x_flip][y_flip] = current_player
                if flips:
                    table[startx][starty] = current_player
                break
            else:
                break
            x += x_dir
            y += y_dir

    return table
