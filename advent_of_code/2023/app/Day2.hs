import Text.Regex.TDFA ((=~))
import Data.List (isInfixOf)
import Data.List.Split (splitOn)
import Data.List (group)
import Data.Char (isDigit)

data ColorsPerGame = RGB { red :: Int, green :: Int, blue :: Int }
    deriving (Show)

extractGameIdAndRest :: String -> (Int, ColorsPerGame)
extractGameIdAndRest line =
  let
    (_, _, cubes, gameId) = line =~ "Game ([[:digit:]]+):" :: (String, String, String, [String])
    gameId' = read $ head gameId :: Int
    formattedCubes = cubes =~ "[[:digit:]]+ [a-z]+" :: [[String]]
    colors = interpretCubes $ concat formattedCubes
  in
    (gameId', colors)

getMaxForColor :: String -> [String] -> Int
getMaxForColor color xs = maximum $ map (read . takeWhile (\ c -> isDigit c)) $ filter (isInfixOf color) xs

interpretCubes :: [String] -> ColorsPerGame
interpretCubes xs = RGB red green blue
  where
    red   = getMaxForColor "red" xs
    green = getMaxForColor "green" xs
    blue  = getMaxForColor "blue" xs

-- Used for Part 1
isPossible :: (Int, ColorsPerGame) -> Int
isPossible (gameId, colors) = 
  if red colors <= 12 &&  green colors <= 13 && blue colors <= 14
    then gameId
    else 0

-- Used for Part 2
powerFewestCubes :: (Int, ColorsPerGame) -> Int
powerFewestCubes (gameId, colors) = red colors * green colors * blue colors


main :: IO ()
main = do
  input <- readFile "./inputs/day2.input"
  let input' = lines input
  putStrLn $ "Part 1: " ++ show (sum $ map (isPossible . extractGameIdAndRest) input')
  putStrLn $ "Part 2: " ++ show (sum $ map (powerFewestCubes . extractGameIdAndRest) input')