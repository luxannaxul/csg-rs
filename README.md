very rusty tool for generating context-sensitive-grammar. example usage:

anywhere in this directory run
>>>cargo run --quiet --release

then you can enter your variables by pressing the corresponding key

variables..
S
A
B
X

press space or enter to get to terminals

terminals..
a
b

its the same thing


prodcutions should be entered in the following format (spaces get ignored completly)
use an empty field on the right side to encode the empty word.
seperate fields on the right with pipe '|'
only use the same pattern on the left side ONCE
enter twice to continue


productions..
S-> aAS |bBS|   X
 Aa ->aA
Bb -> bB
Ba -> aB
Ab -> bA
AX -> Xa
BX -> Xb
X ->

enter your start point

start variable..
S

options are:
(0) exit
(1) generate words of max production depth n

pick an option
and if prompted for arguments, provide them

n..
16

here are all possible words for example, the above defined grammar can prodjuce with at most 16 consecutive rules applied

"" "bb" "aa" "abab" "aaaa" "bbbb" "baba" "babbab" "abaaba" "bbabba" "aaaaaa" "bbbbbb" "baabaa" "aabaab" "abbabb" "abbbabbb" "bbbbbbbb" "baabbaab" "abababab" "aaabaaab" "abaaabaa" "aabbaabb" "aaaaaaaa" "aabaaaba" "bbbabbba" "babbbabb" "abbaabba" "babababa" "bbaabbaa" "bbabbbab" "baaabaaa"


options are:
(0) exit
(1) generate words of max production depth n
