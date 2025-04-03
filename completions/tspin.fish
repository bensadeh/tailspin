complete -c tspin -l config-path -d 'Provide a custom path to a configuration file' -r -F
complete -c tspin -s c -l listen-command -d 'Capture the output (stdout) of a command and view it in `less`' -r
complete -c tspin -l words-red -d 'Highlight the provided words in red' -r
complete -c tspin -l words-green -d 'Highlight the provided words in green' -r
complete -c tspin -l words-yellow -d 'Highlight the provided words in yellow' -r
complete -c tspin -l words-blue -d 'Highlight the provided words in blue' -r
complete -c tspin -l words-magenta -d 'Highlight the provided words in magenta' -r
complete -c tspin -l words-cyan -d 'Highlight the provided words in cyan' -r
complete -c tspin -l enable -d 'Enable specific highlighters' -r -f -a "numbers\t''
urls\t''
pointers\t''
dates\t''
paths\t''
quotes\t''
key-value-pairs\t''
uuids\t''
ip-addresses\t''
processes\t''
json\t''"
complete -c tspin -l disable -d 'Disable specific highlighters' -r -f -a "numbers\t''
urls\t''
pointers\t''
dates\t''
paths\t''
quotes\t''
key-value-pairs\t''
uuids\t''
ip-addresses\t''
processes\t''
json\t''"
complete -c tspin -s f -l follow -d 'Follow the contents of a file'
complete -c tspin -s p -l print -d 'Print the output to stdout'
complete -c tspin -l no-builtin-keywords -d 'Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)'
complete -c tspin -l generate-bash-completions -d 'Print bash completions to stdout'
complete -c tspin -l generate-fish-completions -d 'Print fish completions to stdout'
complete -c tspin -l generate-zsh-completions -d 'Print zsh completions to stdout'
complete -c tspin -s h -l help -d 'Print help'
complete -c tspin -s V -l version -d 'Print version'
