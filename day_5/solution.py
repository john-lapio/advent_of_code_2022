class ElfExpedition:
    """Advent of Code day 5."""

    def __init__(self):
        self.ships: list[list[str]] = [[] for i in range(9)]

    def parse_input(self, order_type: str):
        """Read in the puzzle input."""
        contents = open("puzzle.txt", "r").readlines()
        for line in contents:
            if not "move" in line:
                self.build_cargo_ships(line)
            else:
                self.parse_commands(line, order_type)

    def build_cargo_ships(self, line):
        """Read the contents of the puzzle input and dump the Elf crates into a list
        of lists."""
        count = 0
        line = list(line)
        for idx in range(len(line)):
            if idx % 4 == 0 or line and idx >= len(line) - 1:
                for character in line[idx - 4 : idx]:
                    if character.isalpha():
                        self.ships[count - 1].append(f"{character}")
                count += 1

    def parse_commands(self, line, order_type):
        """Read and implement each move command to each of the lists."""
        command = line.replace("\n", "").split(" ")
        amount = int(command[1])
        source_ship = int(command[3]) - 1
        destination_ship = int(command[5]) - 1
        if order_type == "reverse_order":
            self.reverse_order(amount, source_ship, destination_ship)
        elif order_type == "ordered":
            self.ordered(amount, source_ship, destination_ship)

    def reverse_order(self, amount, source_ship, destination_ship):
        "Run the individual move command."
        cargo = self.ships[source_ship][:amount]
        self.ships[destination_ship] = (
            list(reversed(cargo)) + self.ships[destination_ship]
        )
        self.ships[source_ship] = self.ships[source_ship][
            amount : len(self.ships[source_ship])
        ]

    def ordered(self, amount, source_ship, destination_ship):
        "Run the individual move command where the crates maintain their order."
        cargo = self.ships[source_ship][:amount]
        self.ships[destination_ship] = cargo + self.ships[destination_ship]
        self.ships[source_ship] = self.ships[source_ship][
            amount : len(self.ships[source_ship])
        ]

    def get_top_crate(self):
        """Retrieve the top crate of each ship."""
        print("".join([ship[0] for ship in self.ships]))


def part1():
    """Stack the ship crates one at a time or in reverse order."""
    elf_expedition = ElfExpedition()
    elf_expedition.parse_input("reverse_order")
    elf_expedition.get_top_crate()


def part2():
    """Stack the ship crates in order."""
    elf_expedition = ElfExpedition()
    elf_expedition.parse_input("ordered")
    elf_expedition.get_top_crate()


if __name__ == "__main__":
    part1()
    part2()
