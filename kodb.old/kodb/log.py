import inspect
from kodb.utils import style, right_align


LEVELS = {
    "DEBUG": 5,
    "D": 5,
    "INFO": 4,
    "I": 4,
    "SUCCESS": 3,
    "✓": 3,
    "WARNING": 2,
    "W": 2,
    "ERROR": 1,
    "E": 1,
    "CRITICAL": 0,
    "C": 0
}

class Message:
    def __init__(self, level):
        try:
            # Extract log level from string
            self.level = LEVELS[level.upper()]
        except (KeyError, AttributeError):
            # If key does not match, check if log_level is a valid int or else set the level to DEBUG
            self.level = level if type(level) == int and abs(level) < 5 else 4


    def set_level(self, level):
        self.__init__(level)


    def print_message(self, msg, level_name, *styles, file=None):
        if self.level >= LEVELS[level_name]:
            if file:
                # If there's room, then first print the name of the calling file right aligned
                left_align_len = len(level_name) + 2 + len(msg)
                print(right_align(f"({file})", left_align_len=left_align_len), end="\r")

            # Then print the message with the message level styled
            print("[" + style(level_name, *styles) + "]" + ":", msg)


    def debug(self, msg):
        from_frame = inspect.stack()[1]
        file = inspect.getfile(from_frame[0])
        self.print_message(msg, "D", "bold", file=file)


    def info(self, msg):
        self.print_message(msg, "I", "bold", "blue")


    def success(self, msg):
        self.print_message(msg, "✓", "bold", "green")


    def warning(self, msg):
        self.print_message(msg, "W", "bold", "yellow")


    def error(self, msg):
        self.print_message(msg, "E", "bold", "red")


    def critical(self, msg):
        from_frame = inspect.stack()[1]
        file = inspect.getfile(from_frame[0])
        self.print_message(msg, "C", "bold", "red", "underline", file=file)
