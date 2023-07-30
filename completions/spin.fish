complete -c spin -s c -l config-path -d 'Path to a custom configuration file' -r
complete -c spin -s l -l listen-command -d 'Listens to stdout of the provided command and traps interrupt events from reaching the command' -r
complete -c spin -l z-generate-shell-completions -d 'Print completions to stdout' -r
complete -c spin -s f -l follow -d 'Follow (tail) the contents of the file'
complete -c spin -s p -l print -d 'Print the output to stdout'
complete -c spin -l create-default-config -d 'Generate a new configuration file'
complete -c spin -l show-default-config -d 'Print the default configuration'
complete -c spin -s h -l help -d 'Print help'
complete -c spin -s V -l version -d 'Print version'
