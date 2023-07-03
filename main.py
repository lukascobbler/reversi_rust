import math
import sys
import copy

from PyQt5.QtWidgets import QMainWindow, QApplication, QLabel, QPushButton, QProgressBar
from PyQt5 import uic
from PyQt5.QtCore import QTimer

from game_logic import constants, utility_functions
import time
import bot


class GUI(QMainWindow):
    _WRITE_CONSTANTS = {
        constants.EMPTY: " ",
        constants.BOT: "🔵",
        constants.PLAYER: "🔴",
    }

    _POSSIBLE_MOVE = "👌"

    def __init__(self):
        super(GUI, self).__init__()
        uic.loadUi("gui_entities/gui.ui", self)

        # Class atributes
        self.table = constants.generate_starting_board()
        self.inverted_players = False
        self.past_states = []

        # Coordinates
        self.button_coordinates = []
        for i in range(8):
            for j in range(8):
                self.button_coordinates.append(self.findChild(QPushButton, f"x{i}y{j}"))

        for button in self.button_coordinates:
            button.setEnabled(False)
            button.released.connect(self.process_coordinate_click)

        # Labels
        self.computer_time_label = self.findChild(QLabel, "time_label")
        self.reached_depth_label = self.findChild(QLabel, "depth_label")

        self.num_player_disks = self.findChild(QLabel, "num_player_disks_label")
        self.num_bot_disks = self.findChild(QLabel, "num_bot_disks_label")

        self.winner_label = self.findChild(QLabel, "winner_label")
        self.status_bar = self.findChild(QProgressBar, "status_bar")

        # Buttons
        self.display_player_button = self.findChild(QPushButton, "display_player_button")
        self.display_bot_button = self.findChild(QPushButton, "display_bot_button")

        self.reset_button = self.findChild(QPushButton, "reset_button")
        self.swap_players_button = self.findChild(QPushButton, "swap_players_button")

        self.undo_move_button = self.findChild(QPushButton, "undo_move_button")

        # Setting label texts
        self.display_player_button.setText(GUI._WRITE_CONSTANTS[constants.PLAYER])
        self.display_bot_button.setText(GUI._WRITE_CONSTANTS[constants.BOT])

        self.num_player_disks.setText("2")
        self.num_bot_disks.setText("2")

        self.winner_label.setText("")

        # Assigning functionality to buttons
        self.reset_button.released.connect(self.reset_table)
        self.swap_players_button.released.connect(self.swap_players)

        self.undo_move_button.released.connect(self.undo_move)

        # Game preparation
        self.display_table(utility_functions.all_legal_moves(self.table, constants.PLAYER))

        self.show()

    def display_table(self, possible_moves):
        for button in self.button_coordinates:
            button.setEnabled(False)

        for i in range(8):
            for j in range(8):
                if (i, j) in possible_moves:
                    self.button_coordinates[i * 8 + j].setText(GUI._POSSIBLE_MOVE)
                    self.button_coordinates[i * 8 + j].setEnabled(True)
                else:
                    self.button_coordinates[i * 8 + j].setText(GUI._WRITE_CONSTANTS[self.table[i][j]])

    def process_coordinate_click(self):
        self.past_states.append((copy.deepcopy(self.table), self.status_bar.value()))
        pressed_coordinate_name = self.sender().objectName()
        x = int(pressed_coordinate_name[1])
        y = int(pressed_coordinate_name[3])
        self.player_move(x, y)

    def player_move(self, x, y):
        self.table = utility_functions.next_table(self.table, x, y, constants.PLAYER)
        self.display_table([])

        if not self.update_result(constants.BOT):
            QTimer.singleShot(300, self.bot_move)

    def bot_move(self):
        passed_time, reached_depth, best_found_score = self._bot_move_timed()
        self.display_table(utility_functions.all_legal_moves(self.table, constants.PLAYER))

        self.computer_time_label.setText(f"Bot ({passed_time:.4f}s)")
        self.reached_depth_label.setText(f"Depth ({reached_depth})")
        self.update_result(constants.PLAYER)

        def get_player_condition(score):
            if self.inverted_players:
                return score > 0
            return score < 0

        if best_found_score == math.inf:
            bar_value = 100 if get_player_condition(best_found_score) else 0
        elif best_found_score == -math.inf:
            bar_value = 0 if get_player_condition(best_found_score) else 100
        else:
            bar_value = 50
            bar_value += best_found_score / 100_000 * 50 if get_player_condition(best_found_score) \
                else -(best_found_score / 100_000 * 50)

        self.status_bar.setValue(math.floor(bar_value))

    def _bot_move_timed(self):
        st = time.process_time()
        self.table, reached_depth, best_found_score = bot.computer_turn(self.table, constants.PLAYER, constants.BOT)
        en = time.process_time()

        return (en - st), reached_depth, best_found_score

    def update_result(self, current_player):
        k, i = utility_functions.num_disks(self.table)
        self.num_player_disks.setText(f"{i}")
        self.num_bot_disks.setText(f"{k}")
        end = utility_functions.is_terminal(self.table, current_player)
        if end:
            if k > i:
                self.winner_label.setText("<font color=#FF0000 size=4>The bot won!</font>")
            elif k < i:
                self.winner_label.setText("<font color=#00FF00 size=4>The player won!</font>")
            else:
                self.winner_label.setText("<font color=#EBCA13 size=4>It's a tie!</font>")

        return end

    def reset_table(self):
        self.table = constants.generate_starting_board()
        self.past_states = []
        self.display_table(utility_functions.all_legal_moves(self.table, constants.PLAYER))
        self.computer_time_label.setText("Bot (0.0s)")
        self.reached_depth_label.setText("Depth (0)")
        self.winner_label.setText("")
        self.num_player_disks.setText("2")
        self.num_bot_disks.setText("2")
        self.display_player_button.setText(GUI._WRITE_CONSTANTS[constants.PLAYER])
        self.display_bot_button.setText(GUI._WRITE_CONSTANTS[constants.BOT])
        self.status_bar.setValue(50)

    def undo_move(self):
        try:
            self.table, old_status_value = self.past_states.pop()
            self.status_bar.setValue(old_status_value)
        except IndexError:
            pass
        self.display_table(utility_functions.all_legal_moves(self.table, constants.PLAYER))
        self.winner_label.setText("")
        self.update_result(constants.PLAYER)

    def swap_players(self):
        if self.inverted_players:
            constants.set_player_black()
        else:
            constants.set_player_white()

        self.inverted_players = not self.inverted_players
        self.reset_table()


if __name__ == '__main__':
    app = QApplication(sys.argv)
    MainWindow = GUI()
    app.exec_()
    sys.exit(0)
