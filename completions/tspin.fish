complete -c tspin -l config-path -d 'Provide a custom path to a configuration file' -r
complete -c tspin -s c -l listen-command -d 'Capture the output (stdout) of a command and view it in `less`' -r
complete -c tspin -l words-red -d 'Highlight the provided words in red' -r
complete -c tspin -l words-green -d 'Highlight the provided words in green' -r
complete -c tspin -l words-yellow -d 'Highlight the provided words in yellow' -r
complete -c tspin -l words-blue -d 'Highlight the provided words in blue' -r
complete -c tspin -l words-magenta -d 'Highlight the provided words in magenta' -r
complete -c tspin -l words-cyan -d 'Highlight the provided words in cyan' -r
complete -c tspin -l hidden-generate-shell-completions -d 'Print completions to stdout' -r
complete -c tspin -s f -l follow -d 'Follow the contents of a file'
complete -c tspin -s e -l start-at-end -d 'Start at the end of the file'
complete -c tspin -s p -l print -d 'Print the output to stdout'
complete -c tspin -l enable-numbers -d 'Enable the highlighting of numbers'
complete -c tspin -l enable-dates -d 'Enable the highlighting of dates'
complete -c tspin -l enable-urls -d 'Enable the highlighting of URLs'
complete -c tspin -l enable-paths -d 'Enable the highlighting of paths'
complete -c tspin -l enable-quotes -d 'Enable the highlighting of quotes'
complete -c tspin -l enable-key-value-pairs -d 'Enable the highlighting of key value pairs'
complete -c tspin -l enable-uuids -d 'Enable the highlighting of UUIDs'
complete -c tspin -l enable-ip-addresses -d 'Enable the highlighting of IP addresses'
complete -c tspin -l enable-pointers -d 'Enable the highlighting of pointers'
complete -c tspin -l enable-processes -d 'Enable the highlighting of unix processes'
complete -c tspin -l enable-json -d 'Enable the highlighting of JSON'
complete -c tspin -l disable-numbers -d 'Disable the highlighting of numbers'
complete -c tspin -l disable-dates -d 'Disable the highlighting of dates'
complete -c tspin -l disable-urls -d 'Disable the highlighting of URLs'
complete -c tspin -l disable-paths -d 'Disable the highlighting of paths'
complete -c tspin -l disable-quotes -d 'Disable the highlighting of quotes'
complete -c tspin -l disable-key-value-pairs -d 'Disable the highlighting of key value pairs'
complete -c tspin -l disable-uuids -d 'Disable the highlighting of UUIDs'
complete -c tspin -l disable-ip-addresses -d 'Disable the highlighting of IP addresses'
complete -c tspin -l disable-pointers -d 'Disable the highlighting of pointers'
complete -c tspin -l disable-processes -d 'Disable the highlighting of unix processes'
complete -c tspin -l disable-json -d 'Disable the highlighting of JSON'
complete -c tspin -l no-builtin-keywords -d 'Disable the highlighting of all builtin keyword groups (booleans, nulls, log severities and common REST verbs)'
complete -c tspin -l suppress-output -d 'Suppress all output (for debugging and benchmarking)'
complete -c tspin -s h -l help -d 'Print help'
complete -c tspin -s V -l version -d 'Print version'
