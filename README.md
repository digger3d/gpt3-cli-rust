# CLI tool to call the GPT-3 API

This CLI allows you to quickly generate GPT-3 generated text through the command line. Compile it using the rust toolset.

## Options

    Usage: gpt3 [OPTIONS] <PROMPT>

    Arguments:
    <PROMPT>  The prompt to use for GPT-3

    Options:
    -m, --model <MODEL>              Name of the model to use [default: text-davinci-003]
    -k, --key <KEY>                  The API key to use. Optional if you set it already in the config file
    -s, --store                      Store the API key passed as an argument in the config file
    -x, --max-tokens <MAX_TOKENS>    Max number of tokens to return [default: 256]
    -t, --temperature <TEMPERATURE>  Temperature to use [default: 0.7]
    -v, --verbose                    Enable verbose mode
    -h, --help                       Print help information
    -V, --version                    Print version information
