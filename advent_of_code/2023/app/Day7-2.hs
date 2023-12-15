import Text.Regex.TDFA ((=~))
import Data.Function (on)
import Data.Char (isDigit, digitToInt)
import Data.List
import Data.Bifunctor
import Data.Ord

data HandType = HighCard | OnePair | TwoPair | ThreeOfAKind | FullHouse | FourOfAKind | FiveOfAKind deriving (Enum, Show, Eq, Ord)

makeTuple :: [String] -> (String, String)
makeTuple [x, y] = (x, y)

cardToNumber :: Char -> Int
cardToNumber c
  | c == 'A' = 14
  | c == 'K' = 13
  | c == 'Q' = 12
  | c == 'J' = 1
  | c == 'T' = 10
  | otherwise = digitToInt c

convertHandToNumber :: [Char] -> [Int]
convertHandToNumber = map cardToNumber

convertHandToOccurenceCounts :: [Int] -> [Int]
convertHandToOccurenceCounts = sortBy (comparing Down) . map length . group . sort . filter (/= 1)

identifyHandType :: [Int] -> HandType
identifyHandType hand 
  | null occurencesWithoutJokers = FiveOfAKind
  | occurences == [5] = FiveOfAKind
  | occurences == [4,1] = FourOfAKind
  | occurences == [3, 2] = FullHouse
  | occurences == [3, 1, 1] = ThreeOfAKind
  | occurences == [2, 2, 1] = TwoPair
  | occurences == [2, 1, 1, 1] = OnePair
  | occurences == [1, 1, 1, 1, 1] = HighCard
  where
    occurencesWithoutJokers = convertHandToOccurenceCounts hand
    occurences = (head occurencesWithoutJokers + 5 - sum occurencesWithoutJokers): tail occurencesWithoutJokers


partOneSortingFunction :: [Int] -> [Int] -> Ordering
partOneSortingFunction lhs rhs 
  | lhsHandType == rhsHandType = compare lhs rhs
  | otherwise = compare lhsHandType rhsHandType
  where
    lhsHandType = identifyHandType lhs
    rhsHandType = identifyHandType rhs

accumulate :: [Int] -> Int -> Int
accumulate bids index =
  if null bids
    then 0
    else index * head bids + accumulate (tail bids) (index + 1)

    
main :: IO ()
main = do
  input <- readFile "./inputs/day7.input"
  let
    input' = lines input :: [String]
    tupledInput = map (makeTuple . words) input'
    handConvertedToNumber = fmap (first convertHandToNumber) tupledInput
    bidsOrdered = map (\(_, bid) -> read bid) (sortBy (partOneSortingFunction `on` fst) handConvertedToNumber) :: [Int]

  putStrLn $ "Part 2: " ++ show (accumulate bidsOrdered 1)
