import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn main() {
  let input = result.unwrap(simplifile.read("input.txt"), "invalid")
  let part_1 =
    string.split(input, "\n")
    |> list.map(string.trim)
    |> list.map(int.parse)
    |> list.chunk(result.is_ok)
    |> list.filter(fn(a) {
      case a {
        [Error(Nil)] -> False
        _ -> True
      }
    })
    |> list.map(fn(l) { list.map(l, result.unwrap(_, 0)) })
    |> list.map(fn(l) { list.fold(l, 0, int.add) })
    |> list.fold(0, int.max)

  io.debug(part_1)
}
