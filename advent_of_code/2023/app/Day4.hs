import Text.Regex.TDFA ((=~))
import Data.List (intersect)
import Data.List (nub)

-- Converts a string like "52 94 65 29 89  7 76 80 31 21 78 37 66 69 13 41 93 73 96 16 92 44 62  3 95" to a list of ints
stringToList :: String -> [Int]
stringToList = map read . nub . words

-- Returns the number of matching numbers between two lists
numberOfMatches :: ([Int], [Int]) -> Int
numberOfMatches (winningNumbers, actualNumbers) = length $ intersect winningNumbers actualNumbers

-- Convert the two lists from the input
extractNumbersFromLine :: String -> ([Int], [Int])
extractNumbersFromLine line =
  let
    -- break input in groups
    matches = line =~ "(.*)\\: (.*)\\|(.*)" :: [[String]]
    winningNumbers = stringToList $ matches !! 0 !! 2
    actualNumbers = stringToList $ matches !! 0 !! 3
  in
    (winningNumbers, actualNumbers)

-- Convert the number of matches to points (part 1)
matchesToPoints :: Int -> Int
matchesToPoints matches = points
  where
    points =
      if matches > 0
      then 2 ^ (matches - 1)
      else 0


-- Utility function that adds two lists of different size
addListsKeepRemaining :: Num a => [a] -> [a] -> [a]
addListsKeepRemaining [] ys = ys
addListsKeepRemaining xs [] = xs
addListsKeepRemaining (x:xs) (y:ys) = x + y : addListsKeepRemaining xs ys

-- Takes the list of matches and the total number of cards to recursively
-- build a list of total cards (part 2)
partTwo :: [Int] -> [Int] -> [Int]
partTwo matches cardCount =
  if null matches
  then []
  else
    let
      nMatches = head matches
      newCardCount = addListsKeepRemaining (replicate nMatches (head cardCount)) (tail cardCount)
    in
      head cardCount : partTwo (tail matches) newCardCount


main :: IO ()
main = do
  input <- readFile "./inputs/day4.input"
  let
    input' = lines input :: [String]
    matches = map (numberOfMatches . extractNumbersFromLine) input'

  putStrLn $ "Part 1: " ++ show (sum $ map matchesToPoints matches)
  putStrLn $ "Part 2: " ++ show (sum $ partTwo matches (map (\x -> 1) matches))