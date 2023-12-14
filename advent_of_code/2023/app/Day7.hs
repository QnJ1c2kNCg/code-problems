import Text.Regex.TDFA ((=~))
import Data.Function (on)
import Data.Char (isDigit, digitToInt)
import Data.List
import Data.Bifunctor

data HandType = HighCard | OnePair | TwoPair | ThreeOfAKind | FullHouse | FourOfAKind | FiveOfAKind deriving (Enum, Show, Eq, Ord)

makeTuple :: [String] -> (String, String)
makeTuple [x, y] = (x, y)

cardToNumber :: Char -> Int
cardToNumber c
  | c == 'A' = 14
  | c == 'K' = 13
  | c == 'Q' = 12
  | c == 'J' = 11
  | c == 'T' = 10
  | otherwise = digitToInt c

convertHandToNumber :: [Char] -> [Int]
convertHandToNumber = map cardToNumber

convertHandToOccurenceCounts :: [Int] -> [Int]
convertHandToOccurenceCounts = sort . map length . group . sort 

identifyHandType :: [Int] -> HandType
identifyHandType hand 
  | occurences == [5] = FiveOfAKind
  | occurences == [1,4] = FourOfAKind
  | occurences == [2, 3] = FullHouse
  | occurences == [1, 1, 3] = ThreeOfAKind
  | occurences == [1, 2, 2] = TwoPair
  | occurences == [1, 1, 1, 2] = OnePair
  | occurences == [1, 1, 1, 1, 1] = HighCard
  where occurences = convertHandToOccurenceCounts hand

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
    rez = accumulate bidsOrdered 1

  putStrLn $ "Part 1: " ++ show (accumulate bidsOrdered 1)
