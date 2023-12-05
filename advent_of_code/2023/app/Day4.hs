import Text.Regex.TDFA ((=~))
import Data.List (intersect)
import Data.List (nub)

stringToList :: String -> [Int]
stringToList = map read . nub . words

numberOfMatches :: [Int] -> [Int] -> Int
numberOfMatches winningNumbers actualNumbers = length $ intersect winningNumbers actualNumbers

numbersToPoints :: [Int] -> [Int] -> Int
numbersToPoints winningNumbers actualNumbers =
  let
    nMatchingNumbers = numberOfMatches winningNumbers actualNumbers
    points = if nMatchingNumbers > 0
        then 2 ^ (nMatchingNumbers - 1)
        else 0
  in
    points

lineToPoints :: String -> Int
lineToPoints line = 
  let
    -- break input in groups
    matches = line =~ "(.*)\\: (.*)\\|(.*)" :: [[String]]
    winningNumbers = stringToList $ matches !! 0 !! 2
    actualNumbers = stringToList $ matches !! 0 !! 3

    points = numbersToPoints winningNumbers actualNumbers
  in
    points

main :: IO ()
main = do
  input <- readFile "./inputs/day4.input"
  let input' = lines input

  putStrLn $ "Part 1: " ++ show (sum $ map lineToPoints input')