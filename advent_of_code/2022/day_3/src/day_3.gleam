import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/set
import gleam/string
import simplifile

fn rugsack_split(l: List(String)) -> #(List(String), List(String)) {
  list.split(l, list.length(l) / 2)
}

fn find_common(rugsack: #(List(String), List(String))) {
  let assert Ok(first) =
    set.intersection(set.from_list(rugsack.0), set.from_list(rugsack.1))
    |> set.to_list
    |> list.first

  first
}

fn find_common_with_3_rugsack(l: List(List(String))) -> String {
  let assert Ok(first) =
    list.map(l, set.from_list)
    |> list.reduce(set.intersection)
    |> result.unwrap(set.new())
    |> set.to_list
    |> list.first

  first
}

fn letter_to_score(letter: String) -> Int {
  case letter {
    "a" -> 1
    "b" -> 2
    "c" -> 3
    "d" -> 4
    "e" -> 5
    "f" -> 6
    "g" -> 7
    "h" -> 8
    "i" -> 9
    "j" -> 10
    "k" -> 11
    "l" -> 12
    "m" -> 13
    "n" -> 14
    "o" -> 15
    "p" -> 16
    "q" -> 17
    "r" -> 18
    "s" -> 19
    "t" -> 20
    "u" -> 21
    "v" -> 22
    "w" -> 23
    "x" -> 24
    "y" -> 25
    "z" -> 26
    "A" -> 27
    "B" -> 28
    "C" -> 29
    "D" -> 30
    "E" -> 31
    "F" -> 32
    "G" -> 33
    "H" -> 34
    "I" -> 35
    "J" -> 36
    "K" -> 37
    "L" -> 38
    "M" -> 39
    "N" -> 40
    "O" -> 41
    "P" -> 42
    "Q" -> 43
    "R" -> 44
    "S" -> 45
    "T" -> 46
    "U" -> 47
    "V" -> 48
    "W" -> 49
    "X" -> 50
    "Y" -> 51
    "Z" -> 52
    _ -> 0
  }
}

pub fn main() {
  let assert Ok(input) = simplifile.read("input.txt")
  let part_1 =
    string.split(input, "\n")
    |> list.map(string.to_graphemes)
    |> list.map(rugsack_split)
    |> list.map(find_common)
    |> list.map(letter_to_score)
    |> int.sum
    |> int.to_string

  let part_2 =
    string.split(input, "\n")
    |> list.map(string.to_graphemes)
    |> list.sized_chunk(3)
    |> list.map(find_common_with_3_rugsack)
    |> list.map(letter_to_score)
    |> int.sum
    |> int.to_string
  io.println("Part 1: " <> part_1)
  io.println("Part 2: " <> part_2)
}
