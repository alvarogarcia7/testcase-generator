1→# AGENTS.md
2→
3→## Setup
4→```bash
5→# No setup required yet - empty repository
6→```
7→
8→## Configuration
9→
10→### Editor Settings
11→You can configure which text editor to use by setting environment variables. Copy `.env.example` to `.env` and customize:
12→
13→```bash
14→cp .env.example .env
15→```
16→
17→Supported environment variables:
18→- **EDITOR**: Default text editor for general editing operations (e.g., `vim`, `nano`, `emacs`, `code`)
19→- **VISUAL**: Visual editor for more complex editing tasks (falls back to EDITOR if not set)
20→- **TESTCASE_EDITOR**: Editor specifically for editing test case files (falls back to VISUAL or EDITOR if not set)
21→
22→Example `.env` file:
23→```bash
24→EDITOR=vim
25→VISUAL=code
26→TESTCASE_EDITOR=nano
27→```
28→
29→## Commands
9→- **Build**: make build
10→- **Lint**: make lint
11→- **Test**: make test
12→- **Dev Server**: N/A
13→You must build, test, and lint before committing
14→## Tech Stack
15→- Not yet initialized
16→
17→## Architecture
18→- Repository structure to be determined
19→
20→## Code Style
21→- Follow language-specific conventions once codebase is established
22→
