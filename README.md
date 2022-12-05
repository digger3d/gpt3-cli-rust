# CLI tool to call the GPT-3 API

I made this to quickly generate GPT-3 text from the command line. It's not complete but it works. Compile it using the rust toolset.

## Usage

    Usage: gpt3 [OPTIONS] <PROMPT>

    Arguments:
    <PROMPT>  The prompt to use for GPT-3

    Options:
    -m, --model <MODEL>  Name of the model to use. Defaults to "text-davinci-003" [default: text-davinci-003]
    -k, --key <KEY>      The API key to use. Optional if you set it already in the config file
    -s, --store          Store the API key passed as an argument in the config file
    -v, --verbose        Enable verbose mode
    -h, --help           Print help information
    -V, --version        Print version information
