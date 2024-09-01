import gleam/int
import gleam/io
import gleam/list
import gleam/string
import simplifile

pub fn main() {
  let assert Ok(input) = simplifile.read("input.txt")
  let part_1 =
    string.split(input, "\n")
    |> list.map(fn(row) -> Int {
      case row {
        "A X" -> 1 + 3
        "A Y" -> 2 + 6
        "A Z" -> 3 + 0
        "B X" -> 1 + 0
        "B Y" -> 2 + 3
        "B Z" -> 3 + 6
        "C X" -> 1 + 6
        "C Y" -> 2 + 0
        "C Z" -> 3 + 3
        _ -> 0
      }
    })
    |> list.fold(0, int.add)
    |> int.to_string
  let part_2 =
    string.split(input, "\n")
    |> list.map(fn(row) -> Int {
      case row {
        "A X" -> 3 + 0
        "A Y" -> 1 + 3
        "A Z" -> 2 + 6
        "B X" -> 1 + 0
        "B Y" -> 2 + 3
        "B Z" -> 3 + 6
        "C X" -> 2 + 0
        "C Y" -> 3 + 3
        "C Z" -> 1 + 6
        _ -> 0
      }
    })
    |> list.fold(0, int.add)
    |> int.to_string
  io.println("Part 1: " <> part_1)
  io.println("Part 2: " <> part_2)
}
