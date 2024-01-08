complete -c tspin -s c -l config-path -d 'Provide a custom path to a configuration file' -r
complete -c tspin -s l -l follow-command -d 'Continuously listen to stdout from provided command and prevent interrupt events (Ctrl + C) from reaching the command' -r
complete -c tspin -l words-red -d 'Highlight the provided words in red' -r
complete -c tspin -l words-green -d 'Highlight the provided words in green' -r
complete -c tspin -l words-yellow -d 'Highlight the provided words in yellow' -r
complete -c tspin -l words-blue -d 'Highlight the provided words in blue' -r
complete -c tspin -l words-magenta -d 'Highlight the provided words in magenta' -r
complete -c tspin -l words-cyan -d 'Highlight the provided words in cyan' -r
complete -c tspin -l bucket-size -d 'Set the bucket size for parallel processing' -r
complete -c tspin -l z-generate-shell-completions -d 'Print completions to stdout' -r
complete -c tspin -s f -l follow -d 'Follow the contents of a file'
complete -c tspin -s e -l start-at-end -d 'Start at the end of the file'
complete -c tspin -s p -l print -d 'Print the output to stdout'
complete -c tspin -l disable-builtin-keywords -d 'Disable the highlighting of all builtin keyword groups (booleans, severity and REST)'
complete -c tspin -l disable-booleans -d 'Disable the highlighting of booleans and nulls'
complete -c tspin -l disable-severity -d 'Disable the highlighting of severity levels'
complete -c tspin -l disable-rest -d 'Disable the highlighting of REST verbs'
complete -c tspin -s h -l help -d 'Print help'
complete -c tspin -s V -l version -d 'Print version'
