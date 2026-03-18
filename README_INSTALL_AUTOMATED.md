
## editor 
```
A tool for managing test cases in YAML format

Usage: editor [OPTIONS] <COMMAND>

Commands:
  create                             Create a new test case
  create-general-initial-conditions  
  edit                               Edit an existing test case
  list                               List all test cases
  view                               View a test case
  delete                             Delete a test case
  validate                           Validate test case files
  search                             Search test cases using fuzzy finder
  export                             Export test cases to a test suite file
  import                             Import test cases from a test suite file
  git                                Git operations
  init                               Initialize a new test case repository
  create-interactive                 Create test case interactively with metadata prompts
  build-sequences                    Build test sequences interactively with git commits
  add-steps                          Add steps to a test sequence with git commits
  build-sequences-with-steps         Build test sequences with step collection loops and commits
  complete                           Complete interactive workflow: metadata, conditions, sequences, steps with git commits
  parse-general-conditions           Parse general initial conditions from database with fuzzy search
  parse-initial-conditions           Parse initial conditions from database with fuzzy search
  parse-initial-conditions-complex   Parse initial conditions from database with fuzzy search
  validate-yaml                      Validate a YAML payload against a JSON schema
  export-junit-xml                   Export test runs to JUnit XML format
  validate-junit-xml                 Validate JUnit XML file against XSD schema
  help                               Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>        Path to the test cases directory [default: ./testcases]
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose            Enable verbose output (equivalent to --log-level=info)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## json-escape 
```
Read stdin and perform JSON string escaping

Usage: json-escape [OPTIONS]

Options:
  -t, --test               Test mode: validate that the escaped output is valid JSON when wrapped in quotes
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose            Enable verbose output (equivalent to --log-level=info)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## json-to-yaml 
```
Convert JSON verification output to YAML result files

Usage: json-to-yaml [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input JSON file path

Options:
  -o, --output <OUTPUT>  Output directory for YAML files
  -h, --help             Print help
```
## script-cleanup 
```
Clean script capture output by removing ANSI codes, backspaces, and control characters

Usage: script-cleanup [OPTIONS] --input <INPUT_FILE>

Options:
  -i, --input <INPUT_FILE>    Path to the input file to clean
  -o, --output <OUTPUT_FILE>  Path to the output file (defaults to stdout if not provided)
      --log-level <LEVEL>     Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose               Enable verbose output (equivalent to --log-level=info)
  -h, --help                  Print help
  -V, --version               Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## test-executor 
```
Generate and execute test scripts from YAML test case files

Usage: test-executor [OPTIONS] <COMMAND>

Commands:
  generate         Generate a shell script from a test case YAML file
  execute          Execute a test case by generating and running the script
  hydrate          Hydrate a test case YAML file with variable values from an export file
  generate-export  Generate an export file template from test case hydration_vars declarations
  validate-export  Validate that an export file has all required variables from test case
  list             List all test cases with optional filtering
  resolve          Resolve dependencies in test case YAML files
  help             Print this message or the help of the given subcommand(s)

Options:
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose            Enable verbose output (equivalent to --log-level=info)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## test-orchestrator 
```
Test Orchestrator - Coordinate test case execution with advanced features

Features:
  • Parallel test execution with configurable worker pool
  • Automatic retry with configurable retry policies
  • Real-time progress reporting with live statistics
  • Execution result tracking and report generation
  • Integration with test case storage and verification


Usage: test-orchestrator [OPTIONS] <COMMAND>

Commands:
  run      Execute specific test cases by ID
  run-all  Execute all available test cases
  verify   Verify test execution results from log files
  info     Show orchestrator configuration and status
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>
          Base path for test case storage
          
          [default: testcases]

  -o, --output <OUTPUT>
          Output directory for execution logs and reports
          
          [default: test-output]

      --log-level <LEVEL>
          Set log level (trace, debug, info, warn, error)
          
          [default: info]

      --verbose-logging
          Enable verbose logging (equivalent to --log-level=debug)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## test-plan-documentation-generator-compat 
```
This tool validates that container YAML files generated by the verifier are compatible with the test-plan-doc-gen tool's expected input format. It can validate individual files, batches of files, or test against verifier scenario outputs.

Usage: test-plan-documentation-generator-compat [OPTIONS] [FILE] [COMMAND]

Commands:
  validate                 Validate a single container YAML file
  batch                    Validate multiple container YAML files in a directory
  test-verifier-scenarios  Test against verifier scenario outputs
  report                   Generate a compatibility report for documentation
  help                     Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]
          Container YAML file to validate (if no subcommand provided)

Options:
  -v, --verbose
          Output detailed validation report

      --json
          Output report as JSON

  -h, --help
          Print help (see a summary with '-h')
```
## test-verify 
```
Test verification tool for comparing test execution logs against test case definitions

Usage: test-verify [OPTIONS] <COMMAND>

Commands:
  clean      Verify execution log against test case Clean and display an execution log
  single     Verify a single test execution log against a test case
  batch      Batch verify multiple test execution logs
  parse-log  Parse and display test execution log contents
  help       Print this message or the help of the given subcommand(s)

Options:
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: warn]
      --verbose-logging    Enable verbose logging (equivalent to --log-level=info)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## trm 
```
Manage test run execution records

Usage: trm [OPTIONS] <COMMAND>

Commands:
  list  
  add   
  help  Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>        [default: testcases]
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: info]
  -v, --verbose            Enable verbose output (equivalent to --log-level=debug)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## validate-json 
```
Validate a JSON payload against a JSON schema

Usage: validate-json [OPTIONS] <JSON_FILE> <SCHEMA_FILE>

Arguments:
  <JSON_FILE>    Path to the JSON payload file
  <SCHEMA_FILE>  Path to the JSON schema file

Options:
      --log-level <LEVEL>  Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose            Enable verbose output (equivalent to --log-level=info)
  -h, --help               Print help
  -V, --version            Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## validate-yaml 
```
Validate YAML payloads against a JSON schema

Usage: validate-yaml [OPTIONS] --schema <SCHEMA_FILE> <YAML_FILES>...

Arguments:
  <YAML_FILES>...  Path(s) to the YAML payload file(s)

Options:
  -s, --schema <SCHEMA_FILE>  Path to the JSON schema file
  -w, --watch                 Watch mode - monitor YAML files for changes and re-validate
      --log-level <LEVEL>     Set log level (trace, debug, info, warn, error) [default: warn]
  -v, --verbose               Enable verbose output (equivalent to --log-level=info)
  -h, --help                  Print help
  -V, --version               Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
## verifier 
```
Verify test execution logs against test case definitions

Usage: verifier [OPTIONS]

Options:
  -l, --log <PATH>                 Single-file mode: path to log file
  -c, --test-case <ID>             Single-file mode: test case ID to verify against
  -f, --folder <PATH>              Folder discovery mode: path to folder containing log files
  -F, --format <FORMAT>            Output format (yaml or json) [default: yaml]
  -o, --output <PATH>              Output file path (optional, defaults to stdout)
  -d, --test-case-dir <DIR>        Path to test case storage directory [default: testcases]
      --log-level <LEVEL>          Set log level (trace, debug, info, warn, error) [default: info]
  -v, --verbose                    Enable verbose output (equivalent to --log-level=debug)
  -m, --match-strategy <STRATEGY>  Match strategy for verification (exact, regex, contains, or precomputed) [default: exact]
  -h, --help                       Print help
  -V, --version                    Print version

ENVIRONMENT VARIABLES:
    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level
```
