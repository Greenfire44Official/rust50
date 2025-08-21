#!/bin/bash

# # Check if an argument was provided
# if [ -z "$1" ]; then
#     echo "Usage: $0 <your_argument>"
#     exit 1
# fi

# # Access the first argument using $1
# # echo "You provided the argument: $1"

# cd tmp/$1
# mkdir -p c_to_rust/$1
# git mv -k * c_to_rust/$1 ## -k is used to ignore the error that it can't move the c_to_rust directory
# git mv -k .* c_to_rust/$1 ## .* to move .gitignore and other dot files
# git commit -m "chore: move all files into c_to_rust/$1"

git remote add -f credit tmp/credit
git remote add -f guessing_game tmp/guessing_game
git remote add -f hello_me tmp/hello_me
git remote add -f hello_cargo tmp/hello_cargo
git remote add -f mario_more tmp/mario_more
git remote add -f my_library tmp/my_library

git merge credit/main --allow-unrelated-histories
git merge guessing_game/main --allow-unrelated-histories
git merge hello_me/main --allow-unrelated-histories
git merge hello_cargo/main --allow-unrelated-histories
git merge mario_more/main --allow-unrelated-histories
git merge my_library/main --allow-unrelated-histories