FROM gitpod/workspace-postgres

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        libpython3.6 \
        rust-lldb \
    && sudo rm -rf /var/lib/apt/lists/*

# Rust specific things
RUN cargo install diesel_cli \
    && rustup component add rustfmt

ENV RUST_LLDB=/usr/bin/lldb-8

# Make zsh the default
RUN brew install zsh
ENV SHELL=zsh

# Add some quality of life to our shell
RUN brew install starship  
RUN brew install fzf
RUN $(brew --prefix)/opt/fzf/install
RUN brew install ripgrep
RUN brew install tldr

# Get some reasonable dotfiles
RUN git clone https://gitlab.com/luminovo/public/dotfiles.git $HOME/.dotfiles
RUN $HOME/.dotfiles/install
