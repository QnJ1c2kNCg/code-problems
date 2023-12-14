import Text.Regex.TDFA ((=~))
import Data.Char (isDigit)

stringToList :: String -> [Int]
stringToList = map read . words

computeDistance :: (Int, Int) -> Int
computeDistance (timeAtSpeed, speed) = speed * timeAtSpeed

nWaysToBeatRecord :: (Int, Int) -> Int
nWaysToBeatRecord (time, distanceToBeat) = length $ filter (>distanceToBeat) allDistances
    where
        allDistances = map computeDistance (zip [0..time] [time, time-1..0])
    
main :: IO ()
main = do
  input <- readFile "./inputs/day6.input"
  let
    input' = lines input :: [String]
    -- Janky, but works
    lineOneMatches = (input' !! 0) =~ "(.*)\\: (.*)" :: [[String]]
    lineTwoMatches = (input' !! 1) =~ "(.*)\\: (.*)" :: [[String]]

    times = stringToList $ lineOneMatches !! 0 !! 2
    distances = stringToList $ lineTwoMatches !! 0 !! 2
    nWaysToBeatRecords = map nWaysToBeatRecord (zip times distances)
    bigTime = read $ filter isDigit (lineOneMatches !! 0 !! 2) :: Int
    bigDistance = read $ filter isDigit (lineTwoMatches !! 0 !! 2) :: Int

  putStrLn $ "Part 1: " ++ show (foldr (*)  1 nWaysToBeatRecords)
  putStrLn $ "Part 2: " ++ show (nWaysToBeatRecord (bigTime, bigDistance))
