defmodule AdventOfCode.Day01 do
  def part1(args) do
    args
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn s -> 
    s |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.sum
    end)
    |> Enum.max
  end

  def part2(args) do
    args
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn s -> 
    s |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.sum
    end)
    |> Enum.max
  end
end
