# Navigate to this script’s parent directory
SCRIPTPATH="$( cd "$( dirname "$0" )" && pwd -P )"
cd "$SCRIPTPATH"/..

# Build steps
cd test
bash build-cli.sh
bash build-demo.sh

./surrealism run --fnc llm_question --arg "'how are you?'" --arg "'google/gemma-7b'" --arg "100" --arg "'google--gemma-7b'" demo.surli

# # If “llm” was supplied as a CLI argument, run Surrealism; otherwise just say we built
# if [[ " $@ " =~ [[:space:]]llm[[:space:]] ]]; then
#   ./surrealism run --fnc js_support_agent_sentiment --arg "'how are you?'" --arg "'google--gemma-7b'" demo.surli
# else
#   echo "just building"
# fi