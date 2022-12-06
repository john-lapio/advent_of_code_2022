from typing import Optional


class TuningTrouble:
    """Advent of Code day 6."""

    def __init__(self, char_count: int):
        self.signal: str = ""
        self.char_count: int = char_count
        self.signal_iteration: int = 0
        self.signal_marker: Optional[int] = None

    def read_puzzle_input(self) -> None:
        """Read in the contents of the puzzle input."""
        contents: list[str] = open("puzzle.txt", "r").readlines()
        self.signal = contents[0]

    def check_signal(self) -> None:
        """Iterate through the signal in groups of 4 characters."""
        for i, value in enumerate(self.signal):
            self.signal_marker = self.check_for_marker(
                self.signal[i : i + self.char_count]
            )
            if self.signal_marker is not None:
                break

    def check_for_marker(self, signal_section) -> Optional[int]:
        """Check each 4-character section of signal for a set of unique letters."""
        if not len(set(signal_section)) == self.char_count:
            self.signal_iteration += 1
            return None
        else:
            return self.signal_iteration + self.char_count


def part1():
    """Check for the signal marker per every 4 characters."""
    tt = TuningTrouble(4)
    tt.read_puzzle_input()
    tt.check_signal()
    print(tt.signal_marker)


def part2():
    """Check for the signal marker per every 14 characters."""
    tt = TuningTrouble(14)
    tt.read_puzzle_input()
    tt.check_signal()
    print(tt.signal_marker)


if __name__ == "__main__":
    part1()
    part2()
