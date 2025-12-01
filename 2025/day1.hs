import Data.List

parseInput :: String -> [Int]
parseInput = map rotationToInt . lines

rotationToInt :: [Char] -> Int
rotationToInt ('L' : num) = -(read num)
rotationToInt ('R' : num) = read num

decodePassword' :: [Int] -> Int -> Int -> Int
decodePassword' [] acc zeroes = zeroes
decodePassword' (x : xs) acc zeroes =
  let updatedAcc = mod (acc + x) 100
      z = if updatedAcc == 0 then zeroes + 1 else zeroes
   in decodePassword' xs updatedAcc z

decodePassword2' :: [Int] -> Int -> Int -> Int
decodePassword2' [] acc zeroes = zeroes
decodePassword2' (x : xs) acc zeroes =
  let updatedAcc = mod (acc + x) 100
      rotations =
        if acc + x <= 0
          then
            abs (quot (acc + x) 100) + if acc == 0 then 0 else 1
          else
            div (acc + x) 100
   in decodePassword2' xs updatedAcc zeroes + rotations

decodePasswordPt1 input = decodePassword' input 50 0

decodePasswordPt2 input = decodePassword2' input 50 0

main :: IO ()
main = do
  input <- readFile "./inputs/day1.txt"
  print . decodePasswordPt2 $ parseInput input
