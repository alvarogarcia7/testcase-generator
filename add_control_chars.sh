#!/bin/bash
# Script to add actual control characters to the example files

set -e

echo "Creating cleanup.txt with binary control characters..."

# Use printf to write the file with actual binary control characters
printf 'Script started on 2024-01-22 14:30:45-0800
$ # Cleaning up old log files
$ cd /var/log
$ lls\x08\x08s -lah | grep "\\.log$"
-rw-r--r-- 1 root  root   4.2K Jan 22 10:15 application.log
-rw-r--r-- 1 root  root    12K Jan 22 14:25 system.log
-rw-r--r-- 1 root  root   890K Jan 21 23:59 access.log
$ # Remove old logs
$ rm -rf *.logg\x08
rm: cannot remove '"'"'*.logg'"'"': No such file or directory\x07
$ \x1b[A\x1b[Krm -rf *.log
$ ls -lah
total 8.0K
drwxr-xr-x  2 root root 4.0K Jan 22 14:31 .
drwxr-xr-x 14 root root 4.0K Jan 22 09:00 ..
$ ehco "Cleanup complete"\x07
ehco: command not found
$ \x1b[A\x1b[Kecho "Cleanup complete"
Cleanup complete
$ # Check disk space
$ df -hh\x08
df: invalid option -- '"'"'h'"'"'
Try '"'"'df --help'"'"' for more information.
$ \x1b[A\x1b[Bdf -h
Filesystem      Size  Used Avail Use%% Mounted on
/dev/sda1        50G   25G   23G  52%% /
tmpfs           7.8G     0  7.8G   0%% /dev/shm
$ \x1b[1m\x1b[32mSuccess!\x1b[0m Cleanup complete
$ # Verify backup directory exists
$ ls /backup/logsss\x08\x08\x08
ls: cannot access '"'"'/backup/logsss'"'"': No such file or directory\x07
$ \x1b[A\x1b[Kls /backup/logs
2024-01-20.tar.gz  2024-01-21.tar.gz
$ # Create archive with today'"'"'s logs
$ tar czf /backup/logs/2024-01-22.tar.gz /var/log/*.logg\x08
tar: /var/log/*.logg: Cannot stat: No such file or directory\x07
tar: Exiting with failure status due to previous errors\x07
$ \x1b[A\x1b[Ktar czf /backup/logs/2024-01-22.tar.gz /var/log/*.log
$ echo "Archive created successfully"
Archive created successfully
$ exit
Script done on 2024-01-22 14:32:18-0800
' > cleanup.txt

echo "Created cleanup.txt"

echo "Creating errors.txt with binary control characters..."

printf 'Script started on 2024-01-22 15:45:22-0800
$ # Running test suite
$ pythonn\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08python test_runner.py
pythonn: command not found\x07
$ python test_runner.py
\x1b[?25l  Running tests...
\x1b[2K\x1b[1A\x1b[2K  \x1b[32m✓\x1b[0m Test 1: Authentication... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m✓\x1b[0m Test 2: Database connection... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[31m✗\x1b[0m Test 3: API endpoint... \x1b[31mFAIL\x1b[0m
\x1b[2K  \x1b[33m⚠\x1b[0m Test 4: Cache invalidation... \x1b[33mSKIP\x1b[0m
\x1b[2K  \x1b[32m✓\x1b[0m Test 5: Error handling... \x1b[32mPASS\x1b[0m
\x1b[?25h

\x1b[1m\x1b[31mSummary: 3 passed, 1 failed, 1 skipped\x1b[0m

\x1b[31mERROR: Test 3 failed with exception:\x1b[0m
  File "/app/tests/test_api.py", line 45, in test_api_endpoint
    assert response.status_code == 200
AssertionError: expected 200 but got 404

$ # Try to debug the issue
$ curll\x08\x08 -X GET http://localhost:8080/api/users
curll: command not found\x07
$ curl -X GET http://localhost:8080/api/users
curl: (7) Failed to connect to localhost port 8080: Connection refused\x07

$ # Start the server first
$ ./start_server.sh &
[1] 12345
Starting server on port 8080...
\x1b[33m[WARNING]\x1b[0m Loading configuration from default settings
\x1b[32m[INFO]\x1b[0m Server started successfully on \x1b[1mhttp://localhost:8080\x1b[0m

$ curl -X GET http://localhost:8080/api/userss\x08
{"error": "Not found", "path": "/api/userss"}
$ \x1b[A\x1b[Kcurl -X GET http://localhost:8080/api/users
[
  {"id": 1, "name": "Alice", "email": "alice@example.com"},
  {"id": 2, "name": "Bob", "email": "bob@example.com"}
]
$ # Fix the test
$ vim tests/test_api.py
$ python test_runner.py
\x1b[?25l  Running tests...
\x1b[2K\x1b[1A\x1b[2K  \x1b[32m✓\x1b[0m Test 1: Authentication... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m✓\x1b[0m Test 2: Database connection... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m✓\x1b[0m Test 3: API endpoint... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[33m⚠\x1b[0m Test 4: Cache invalidation... \x1b[33mSKIP\x1b[0m
\x1b[2K  \x1b[32m✓\x1b[0m Test 5: Error handling... \x1b[32mPASS\x1b[0m
\x1b[?25h

\x1b[1m\x1b[32mSummary: 4 passed, 0 failed, 1 skipped\x1b[0m

\x1b[1m\x1b[7m\x1b[32m All tests passed! \x1b[0m 

$ # Test database connection
$ psql -h localhost -U admin testdb
psql: error: connection to server at "localhost" (127.0.0.1), port 5432 failed: Connection refused\x07
	Is the server running on that host and accepting TCP/IP connections?
$ sudo systemctl start postgresql
$ psql -h localhost -U admin testdb
Password for user admin: 
psql (14.5)
Type "help" for help.

testdb=# SELECT COUNT(*) FROM userss\x08;
ERROR:  relation "userss" does not exist\x07
LINE 1: SELECT COUNT(*) FROM userss;
                              ^
testdb=# \x1b[A\x1b[KSELECT COUNT(*) FROM users;
 count 
-------
   142
(1 row)

testdb=# \\q
$ kill %%1
[1]+  Terminated              ./start_server.sh
$ exit
Script done on 2024-01-22 15:52:03-0800
' > errors.txt

echo "Created errors.txt"
echo ""
echo "Files created successfully!"
echo ""
echo "Control characters included:"
echo "  - Backspace (0x08) for typo corrections"
echo "  - Bell (0x07) for error notifications"
echo "  - ANSI escape sequences for colors, cursor movement, etc."
echo ""
echo "To view with control characters visible:"
echo "  cat -v cleanup.txt"
echo "  cat -v errors.txt"
