import Data.Char (isDigit)
import Data.List (find)
import Data.List.Split (splitOn)
import Data.Maybe (fromJust)

main :: IO ()
main = do
  input <- readFile "day_1.input"
  let input' = splitOn "\n" input

  let firstDigit' = fromJust . find isDigit
  let mapFn xs = read (firstDigit' xs : [(firstDigit' . reverse) xs]) :: Integer

  print $ sum $ map mapFn input'