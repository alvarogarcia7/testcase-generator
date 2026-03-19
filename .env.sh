[[ -f "Makefile" ]] && [[ 0 -eq $(grep -q "test:" Makefile) ]]

if [[ -d $HOME/Documents/projects/test-plan-documentation-generator ]]; then
	export TEST_PLAN_DOC_GEN="$HOME/Documents/projects/test-plan-documentation-generator/"
	export TEST_PLAN_DOC_GEN_DIR="$HOME/Documents/projects/test-plan-documentation-generator/"
fi
