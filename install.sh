#!/bin/bash

# colors 

GREEN="\x1B[32m"
YELLOW="\x1B[33m"
YELLOW_BRIGHT="\x1B[93m"
BLUE="\x1B[34m"
CYAN="\033[36m"
RED="\033[1;31m"
WHITE="\x1B[37m"
BOLD="\x1B[1m"
RESET="\x1B[0m"

# installing cargo

echo -e "${GREEN}[!]${RESET} Hey! Welcome to the installation of skyfetch."

read -r -p "Have you already installed cargo? [Y/n] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
  echo -e "${GREEN}[!]${RESET} Ok!"
else
   curl https://sh.rustup.rs -sSf | sh
fi

# clone the repository

git clone https://github.com/justleoo/skyfetch
cd skyfetch

# installing

cargo install --path .

# finally

echo -e "${GREEN}[!]${RESET} Yay, skyfetch installed, use 'skyfetch' command in your terminal."
