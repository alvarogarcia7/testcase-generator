[[ -f "Makefile" ]] && [[ 0 -eq $(grep -q "test:" Makefile) ]] && make test
