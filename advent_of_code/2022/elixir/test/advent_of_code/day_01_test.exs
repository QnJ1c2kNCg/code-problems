defmodule AdventOfCode.Day01Test do
  use ExUnit.Case

  import AdventOfCode.Day01

  test "part1" do
    input = File.read!("inputs/day_1.txt")
    result = part1(input)

    assert result
    IO.puts result
  end

  test "part2" do
    input = File.read!("inputs/day_1.txt")
    result = part2(input)

    assert result
  end
end
