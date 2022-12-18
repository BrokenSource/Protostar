#!/usr/bin/bash

# # Install Rustup if not found
if [ -z $(which rustup 2> /dev/null) ]; then
    curl -sSL https://sh.rustup.rs | sh -s -- -y
fi

# Windows fix to find rustup, shouldn't do much on unix
export PATH=$PATH:$USERPROFILE/.cargo/bin

# Download Rust stable
rustup default stable

# Clone bare Protostar Repo
git clone https://www.github.com/BrokenSource/Protostar
cd Protostar

# Init public submodules projects
projects=(
    "Assets"
    # "HypeWord"
    # "PhasorFlow"
    # "ShaderFlow"
    "ViyLine"
)

# Init submodules
for project in "${projects[@]}"; do
    git submodule init $project
    git submodule update $project
done

printf "\n:: Now type (cd Protostar) and run projects with (cargo projectName)"
