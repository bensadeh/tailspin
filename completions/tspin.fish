complete -c tspin -s c -l config-path -d 'Path to a custom configuration file' -r
complete -c tspin -s l -l follow-command -d 'Continuously listens to the stdout of the provided command and prevents interrupt events (Ctrl + C) from reaching the command' -r
complete -c tspin -l z-generate-shell-completions -d 'Print completions to stdout' -r
complete -c tspin -s f -l follow -d 'Follow (tail) the contents of the file'
complete -c tspin -s t -l tail -d 'Start at the end of the file'
complete -c tspin -s p -l print -d 'Print the output to stdout'
complete -c tspin -s h -l help -d 'Print help'
complete -c tspin -s V -l version -d 'Print version'
