#Letterboxed Solver

The purpose of this program is to solve the letterboxed and also give myself an easy way to brush up on my C skills.

## PLAN/TODO:
### PREPARE
1. Use the word pool from https://raw.githubusercontent.com/dwyl/english-words/master/words.txt
2. Prepare for "legal letterboxed words"
   - double letters such as "school" "letter"
   - no captial letter specific pronouns
   - no Hyphenated words
   - no words with numbers in them
   - no words with more than 12 unique letters
   - sort all words into a text file list which starts with the first 2 letters of the word
  
### ATTACK
1. Write a word search algorithm which first goes through each legal instance of legal threads and finds all legal words
2. Write a second word search algorithm which iterates through the legal words and determines which ones would be the best for moving forward (most amount of unique letters or words)
3. String words together and make sure you use all the letters


### PRESENT
1. Write a design document which tells users how to use the program
2. Using args on the program in order to take in words or seperate word pools?
3. GUI? (maybe later)

## User story:
1. As a letterboxed player, I would like to put in the letters from each side and find a string of solvable words which could be used to solve the game quickly
2. As a letterboxed player, I would like the program to work within 10 seconds of searching (Use threading if I need to)
