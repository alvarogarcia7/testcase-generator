
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
  -p, --path <PATH>  Path to the test cases directory [default: ./testcases]
  -v, --verbose      Enable verbose logging
  -h, --help         Print help
  -V, --version      Print version
```
## script-cleanup 
```
Clean script capture output by removing ANSI codes, backspaces, and control characters

Usage: script-cleanup [OPTIONS] --input <INPUT_FILE>

Options:
  -i, --input <INPUT_FILE>    Path to the input file to clean
  -o, --output <OUTPUT_FILE>  Path to the output file (defaults to stdout if not provided)
  -v, --verbose               Enable verbose logging
  -h, --help                  Print help
  -V, --version               Print version
```
## test-executor 
```
Generate and execute test scripts from YAML test case files

Usage: test-executor <COMMAND>

Commands:
  generate  Generate a shell script from a test case YAML file
  execute   Execute a test case by generating and running the script
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
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

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
## test-verify 
```
Test verification tool for comparing test execution logs against test case definitions

Usage: test-verify <COMMAND>

Commands:
  clean      Verify execution log against test case Clean and display an execution log
  single     Verify a single test execution log against a test case
  batch      Batch verify multiple test execution logs
  parse-log  Parse and display test execution log contents
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## testcase-manager 
```
A tool for managing test cases in YAML format

Usage: testcase-manager [OPTIONS] <COMMAND>

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
  -p, --path <PATH>  Path to the test cases directory [default: ./testcases]
  -v, --verbose      Enable verbose logging
  -h, --help         Print help
  -V, --version      Print version
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
  -p, --path <PATH>  [default: testcases]
  -h, --help         Print help
  -V, --version      Print version
```
## validate-json 
```
Validate a JSON payload against a JSON schema

Usage: validate-json [OPTIONS] <JSON_FILE> <SCHEMA_FILE>

Arguments:
  <JSON_FILE>    Path to the JSON payload file
  <SCHEMA_FILE>  Path to the JSON schema file

Options:
  -v, --verbose  Enable verbose logging
  -h, --help     Print help
  -V, --version  Print version
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
  -v, --verbose               Enable verbose logging
  -h, --help                  Print help
  -V, --version               Print version
```
