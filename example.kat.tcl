
# Create a file like this at $HOME/.config/kat/kat.tcl and
# then run `kat` from within a kitty terminal. A shell will
# open. Then run `open <name>` to open a tab at the specified
# path. For example:
#
# % open redox

proj {
    # the name of our tab
    name redox
    # currently unused: a git repo remote url
    git git@gitlab.redox-os.org:redox-os/redox.git
    # the path to cd into when we open a tab
    path /home/coleman/Code/Redox/redox
}

# specify as many proj as you want
