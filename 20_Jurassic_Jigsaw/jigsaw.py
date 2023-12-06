#!/usr/bin/env python3

import itertools
import operator
import sys
from collections import Counter
from dataclasses import dataclass
from functools import cached_property, reduce
from math import isqrt
from pathlib import Path
from typing import Self

n = 144
# n = 9  # for small input
outer_w = isqrt(n)
w = 10


def rotate_right(matrix):
    rotated = [[None] * len(matrix) for _ in range(len(matrix[0]))]
    for y in range(len(rotated)):
        for x in range(len(rotated[0])):
            rotated[y][x] = matrix[len(matrix) - 1 - x][y]
    return rotated


def flip(matrix):
    flipped = [[None] * len(matrix) for _ in range(len(matrix[0]))]
    for y in range(len(flipped)):
        for x in range(len(flipped[0])):
            flipped[y][x] = matrix[x][y]
    return flipped


@dataclass(frozen=True)
class Tile:
    id: int
    rows: list[list[bool]]
    is_flipped: bool = False
    rotations: int = 0

    def __str__(self) -> str:
        header = f"Tile {self.id} ({self.is_flipped=}, {self.rotations=})"
        body = "".join(
            "".join("#" if c else "." for c in row) + "\n" for row in self.rows
        )
        return header + "\n" + body

    def __repr__(self):
        return str(self)

    @classmethod
    def from_lines(cls, lines: list[str]) -> Self:
        assert len(lines) == w + 1
        id = int(lines[0].strip().removeprefix("Tile ").removesuffix(":"))
        for row in lines[1:]:
            assert len(row.strip()) == w
        return cls(id, [[c == "#" for c in row.strip()] for row in lines[1:]])

    @cached_property
    def rotate_right(self) -> Self:
        return type(self)(
            self.id, rotate_right(self.rows), rotations=self.rotations + 1
        )

    @cached_property
    def flip(self) -> Self:
        return type(self)(self.id, flip(self.rows), is_flipped=not self.is_flipped)

    @cached_property
    def top_fingerprint(self) -> int:
        fp = 0
        for b in self.rows[0]:
            fp <<= 1
            fp |= b
        return fp

    @cached_property
    def fingerprints(self) -> tuple[int, int, int, int]:
        return (
            self.top_fingerprint,
            self.rotate_right.rotate_right.rotate_right.top_fingerprint,
            self.rotate_right.rotate_right.top_fingerprint,
            self.rotate_right.top_fingerprint,
        )


def flip_fp(fp: int) -> int:
    flipped = 0
    for _ in range(w):
        flipped <<= 1
        flipped |= fp & 1
        fp >>= 1
    return flipped


def find_and_remove_matching_tile(tiles, fp):
    for i in range(len(tiles)):
        if flip_fp(fp) in tiles[i].fingerprints:
            return tiles.pop(i)
        if flip_fp(fp) in tiles[i].flip.fingerprints:
            return tiles.pop(i).flip
    assert False, "Could not find tile"


def find_tile_with_fp(fp, dir, tiles):
    tile = find_and_remove_matching_tile(tiles, fp)
    while fp != flip_fp(tile.fingerprints[dir]):
        tile = tile.rotate_right
    return tile


def find_tile_right_to(left, tiles):
    return find_tile_with_fp(left.fingerprints[1], 3, tiles)


monster = [
    "                  #",
    "#    ##    ##    ###",
    " #  #  #  #  #  #",
]

monster_offsets = [
    (x, y) for y, row in enumerate(monster) for x, c in enumerate(row) if c == "#"
]


def monster_positions_for(x, y):
    for dx, dy in monster_offsets:
        yield (x + dx, y + dy)


def has_monster(picture, x, y):
    for mx, my in monster_positions_for(x, y):
        if not (0 <= my < len(picture) and 0 <= mx < len(picture[my])):
            return False
        if not picture[my][mx]:
            return False
    return True


def try_it(picture):
    count = 0
    monster_positions = set()
    for y in range(len(picture)):
        for x in range(len(picture[0])):
            if has_monster(picture, x, y):
                count += 1
                monster_positions |= set(monster_positions_for(x, y))
    if count > 0:
        print("monsters:", count)
        total_hashes = sum(b for row in picture for b in row)
        print("roughness:", total_hashes - len(monster_positions))


def main():
    filename = sys.argv[1] if len(sys.argv) > 1 else "input"
    data = (Path(__file__).parent / filename).open().read().strip().split("\n")

    tiles = []
    for i in range(n):
        start = (w + 2) * i
        tiles.append(Tile.from_lines(data[start : start + 1 + w]))

    fp_hist = Counter(
        fp for t in tiles for fp in itertools.chain(t.fingerprints, t.flip.fingerprints)
    )

    fp_fp = lambda t: list(fp_hist[fp] for fp in t.fingerprints)

    corner_tiles = [t for t in tiles if sorted(fp_fp(t)) == [1, 1, 2, 2]]
    edge_tiles = [t for t in tiles if sorted(fp_fp(t)) == [1, 2, 2, 2]]
    inner_tiles = [t for t in tiles if t not in corner_tiles and t not in edge_tiles]

    # Part 1
    print(reduce(operator.mul, (t.id for t in corner_tiles)))

    # Part 2
    rows = []

    # Assemble first row
    first_row = []
    top_left = corner_tiles.pop()
    while not fp_fp(top_left) == [1, 2, 2, 1]:
        top_left = top_left.rotate_right
    first_row.append(top_left)

    for _ in range(outer_w - 2):
        first_row.append(find_tile_right_to(first_row[-1], edge_tiles))

    first_row.append(find_tile_right_to(first_row[-1], corner_tiles))

    rows.append(first_row)

    # Assemble other rows
    for _ in range(outer_w - 2):
        row = []
        left = find_tile_with_fp(rows[-1][0].fingerprints[2], 0, edge_tiles)
        row.append(left)

        for _ in range(outer_w - 2):
            row.append(find_tile_right_to(row[-1], inner_tiles))

        row.append(find_tile_right_to(row[-1], edge_tiles))
        rows.append(row)
    # Assemble final row
    last_row = []
    left = find_tile_with_fp(rows[-1][0].fingerprints[2], 0, corner_tiles)
    last_row.append(left)

    for _ in range(outer_w - 2):
        last_row.append(find_tile_right_to(last_row[-1], edge_tiles))

    last_row.append(find_tile_right_to(last_row[-1], corner_tiles))
    rows.append(last_row)

    assert len(corner_tiles) == len(edge_tiles) == len(inner_tiles) == 0

    # Cut off edges, assemble full picture
    picture = []
    for row in rows:
        for y in range(w - 2):
            picture.append([])
            for tile in row:
                for x in range(w - 2):
                    picture[-1].append(tile.rows[y + 1][x + 1])

    print("Assembled picture:")
    for row in picture:
        for b in row:
            print("#" if b else ".", end="")
        print()

    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)
    picture = flip(picture)
    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)
    picture = rotate_right(picture)
    try_it(picture)


if __name__ == "__main__":
    main()
