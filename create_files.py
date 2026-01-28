#!/usr/bin/env python3
"""Create example script capture files with real control characters."""

# Cleanup.txt content with actual binary control characters
cleanup_content = b'''Script started on 2024-01-22 14:30:45-0800
$ # Cleaning up old log files
$ cd /var/log
$ lls\x08\x08s -lah | grep "\\.log$"
-rw-r--r-- 1 root  root   4.2K Jan 22 10:15 application.log
-rw-r--r-- 1 root  root    12K Jan 22 14:25 system.log
-rw-r--r-- 1 root  root   890K Jan 21 23:59 access.log
$ # Remove old logs
$ rm -rf *.logg\x08
rm: cannot remove '*.logg': No such file or directory
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
df: invalid option -- 'h'
Try 'df --help' for more information.
$ \x1b[A\x1b[Bdf -h
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1        50G   25G   23G  52% /
tmpfs           7.8G     0  7.8G   0% /dev/shm
$ \x1b[1m\x1b[32mSuccess!\x1b[0m Cleanup complete
$ # Test with typos and corrections
$ catt /etc/hostname\x08\x08\x08\x08cat /etc/hostname
my-server.local
$ whoamii\x08\x08\x08\x08\x08whoami
root
$ echho "Test"\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08echo "Test"
Test
$ psss\x08 aux | head\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08ps aux | head
USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root         1  0.0  0.1  22548  3456 ?        Ss   09:00   0:01 /sbin/init
$ \x07\x07# Multiple bell characters for alerts
$ exit
Script done on 2024-01-22 14:32:18-0800
'''

# Errors.txt content with actual binary control characters
errors_content = b'''Script started on 2024-01-22 15:45:22-0800
$ # Running test suite
$ pythonn test_runner.py\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08python test_runner.py
pythonn: command not found
$ python test_runner.py
\x1b[?25l  Running tests...
\x1b[2K\x1b[1A\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 1: Authentication... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 2: Database connection... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[31m\xe2\x9c\x97\x1b[0m Test 3: API endpoint... \x1b[31mFAIL\x1b[0m
\x1b[2K  \x1b[33m\xe2\x9a\xa0\x1b[0m Test 4: Cache invalidation... \x1b[33mSKIP\x1b[0m
\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 5: Error handling... \x1b[32mPASS\x1b[0m
\x1b[?25h

\x1b[1m\x1b[31mSummary: 3 passed, 1 failed, 1 skipped\x1b[0m

\x1b[31mERROR: Test 3 failed with exception:\x1b[0m
  File "/app/tests/test_api.py", line 45, in test_api_endpoint
    assert response.status_code == 200
AssertionError: expected 200 but got 404

$ # Try to debug the issue
$ curll -X GET http://localhost:8080/api/users\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08curl -X GET http://localhost:8080/api/users
curll: command not found
$ curl -X GET http://localhost:8080/api/users
curl: (7) Failed to connect to localhost port 8080: Connection refused
\x07

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
\x1b[2K\x1b[1A\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 1: Authentication... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 2: Database connection... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 3: API endpoint... \x1b[32mPASS\x1b[0m
\x1b[2K  \x1b[33m\xe2\x9a\xa0\x1b[0m Test 4: Cache invalidation... \x1b[33mSKIP\x1b[0m
\x1b[2K  \x1b[32m\xe2\x9c\x93\x1b[0m Test 5: Error handling... \x1b[32mPASS\x1b[0m
\x1b[?25h

\x1b[1m\x1b[32mSummary: 4 passed, 0 failed, 1 skipped\x1b[0m

\x1b[1m\x1b[7m\x1b[32m All tests passed! \x1b[0m 

$ # More typos with corrections
$ gitt status\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08git status
gitt: command not found
$ git status
On branch main
nothing to commit, working tree clean
$ sssh user@server\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08ssh user@server
sssh: command not found
$ \x07# Bell on error
$ kill %1
[1]+  Terminated              ./start_server.sh
$ exit
Script done on 2024-01-22 15:52:03-0800
'''

# Write files
with open('cleanup.txt', 'wb') as f:
    f.write(cleanup_content)

with open('errors.txt', 'wb') as f:
    f.write(errors_content)

print("✓ Created cleanup.txt and errors.txt with actual binary control characters")
print("  • Backspace characters (0x08)")
print("  • Bell characters (0x07)")
print("  • ANSI escape sequences")
print("  • UTF-8 symbols (✓, ✗, ⚠)")
print("  • User typos and corrections")
