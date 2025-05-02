# linux only, free to use software for non-profit usage.

this is a tool for generating words based on a *context sensitive grammar*.

how to use:

1. install *rust* on your system: https://www.rust-lang.org/tools/install

2. install *git* on your system: https://git-scm.com/downloads

3. run in your destination directory>>>git clone https://github.com/luxannaxul/csg-rs.git

4. run >>>cd csg-rs

5. compile and run the program by running >>>cargo run --release

once you see "variables.." just type in your single sign variables. <space> or <enter> to continue

the exact same thing now with your terminals..

how to enter productions:

specify the origin on left, seperate left from right with '->'
list all possible conversions seperated by '|' (pipe) on the right.
to enter <epsilon> (the empty word) just let one field on the right empty.
press <enter> twice to continue.

note: every left side has to be unique, every repetition on the right as well as every <space> gets ignored.

example:

productions..
S -> aAS | bBS|X
Aa-> aA
Ab->bA
Ba->aB
Ba->bB
AX->Xa
BX->Xb
X->
<enter>
<enter>

then you just need to set your start variable and can natrually interact with the menu


