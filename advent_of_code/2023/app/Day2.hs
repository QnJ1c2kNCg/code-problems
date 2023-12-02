import Text.Regex.TDFA ((=~))
import Data.List (isInfixOf)
import Data.List.Split (splitOn)
import Data.List (group)

data ColorsPerGame = RGB { red :: Int, green :: Int, blue :: Int }
    deriving (Show)

-- interpretCubes :: [String] -> ColorsPerGame
-- interpretCubes cubes = 
--   -- for each set in the game, we pick the max
--   RGB { red = 255, green = 0, blue = 128 }

interpretCubes :: [String] -> ([Int], [Int], [Int])
interpretCubes xs = (red, green, blue)
  where
    red   =  group $ takeWhile (\c -> isDigit c) . filter (isInfixOf "red")  xs :: Int
    green = filter (isInfixOf "green") xs
    blue  = filter (isInfixOf "blue") xs

main :: IO ()
main = do
  input <- readFile "./inputs/day2.input"
  let input' = lines input
  
  let first = input' !! 0
  print first
  
  -- Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
  -- let regexPattern = "Game ([[:digit:]]+): ([[:digit:]]+ [a-z]+[, ]?)+"
  let regexPattern = "Game ([[:digit:]]+):"

  let (_, _, cubesAllPulls, gameId) = first =~ regexPattern :: (String, String, String, [String])
  let gameId' = head gameId
  putStrLn ("Game ID:" ++ gameId')
  let cubesAllPulls' = splitOn ";" cubesAllPulls
  let headCubes = head cubesAllPulls'
  let matches = cubesAllPulls =~ "[[:digit:]]+ [a-z]+" :: [[String]]
  print matches

  -- -- The regex pattern with a group capturing the date part
  -- let regexPattern = "Date: (\\d{4}-\\d{2}-\\d{2})"
  
  -- -- Applying the regex and extracting the captured group
  -- let matchResult = inputString =~ regexPattern :: (String, String, String, [String])
  
  -- -- The fourth element of the tuple contains the captured groups
  -- let capturedGroups = if length (snd matchResult) > 0 then head (snd matchResult) else ""
  
  -- putStrLn $ "Input String: " ++ inputString
  -- putStrLn $ "Captured Date: " ++ capturedGroups

  -- print input'