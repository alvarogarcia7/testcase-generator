backlog-browser:
	node ./node_modules/backlog.md/cli.js browser --port $$((RANDOM % 10000 + 1024))
.PHONY: backlog-browser

backlog: backlog-browser
