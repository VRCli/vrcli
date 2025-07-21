#!/bin/bash

# Set UTF-8 encoding
export LANG=UTF-8
export LC_ALL=UTF-8

echo -e "\033[32mFetching online friends...\033[0m"

# Function to run vrcli commands and capture output
run_vrcli() {
    local args="$1"
    local output_file=$(mktemp)
    local error_file=$(mktemp)
    local exit_code
    
    # Run vrcli command and capture output
    vrcli $args > "$output_file" 2> "$error_file"
    exit_code=$?
    
    # Read outputs
    local stdout_content=$(cat "$output_file")
    local stderr_content=$(cat "$error_file")
    
    # Cleanup temp files
    rm -f "$output_file" "$error_file"
    
    # Return results via global variables
    VRCLI_EXIT_CODE=$exit_code
    VRCLI_OUTPUT="$stdout_content"
    VRCLI_ERROR="$stderr_content"
}

# Check if vrcli is available
if ! command -v vrcli &> /dev/null; then
    echo -e "\033[31mError: vrcli command not found\033[0m" >&2
    echo -e "\033[33mRun 'vrcli auth login' and ensure vrcli is in PATH\033[0m" >&2
    exit 1
fi

# Get online friends with location data
run_vrcli "friends list --online --json --show-location"

if [ $VRCLI_EXIT_CODE -ne 0 ]; then
    echo -e "\033[31mFriend fetch failed (Error $VRCLI_EXIT_CODE)\033[0m" >&2
    echo -e "$VRCLI_ERROR" >&2
    echo -e "\033[33mRun 'vrcli auth login' and ensure vrcli in PATH\033[0m" >&2
    exit $VRCLI_EXIT_CODE
fi

# Parse JSON and filter friends
if ! friends_json=$(echo "$VRCLI_OUTPUT" | jq -c '.[] | select(.location and (.location | test("private|offline") | not) and .display_name and (.display_name | test("^\\s*$") | not))' 2>/dev/null); then
    echo -e "\033[31mJSON parse failed\033[0m" >&2
    echo -e "\033[33mRaw output:\033[0m" >&2
    echo "$VRCLI_OUTPUT" >&2
    exit 1
fi

# Check if we have any friends
if [ -z "$friends_json" ]; then
    echo -e "\033[33mNo friends found online in non-private instances.\033[0m" >&2
    exit 0
fi

# Select random friend and get display name
# Convert to array and pick random element
friends_array=()
while IFS= read -r line; do
    friends_array+=("$line")
done <<< "$friends_json"

if [ ${#friends_array[@]} -eq 0 ]; then
    echo -e "\033[33mNo friends found online in non-private instances.\033[0m" >&2
    exit 0
fi

# Get random friend
random_index=$((RANDOM % ${#friends_array[@]}))
selected_friend="${friends_array[$random_index]}"

# Extract and sanitize display name
name=$(echo "$selected_friend" | jq -r '.display_name' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' | sed 's/[^[:print:]]//g')

if [ -z "$name" ]; then
    echo -e "\033[31mFailed to extract friend name\033[0m" >&2
    exit 1
fi

echo -e "\033[32mSending invite request to: $name\033[0m"

# Send invite request
run_vrcli "invite request \"$name\""

if [ $VRCLI_EXIT_CODE -ne 0 ]; then
    echo -e "\033[31mInvite failed (Error $VRCLI_EXIT_CODE)\033[0m" >&2
    echo -e "$VRCLI_ERROR" >&2
    exit $VRCLI_EXIT_CODE
elif [ -n "$VRCLI_OUTPUT" ]; then
    echo "$VRCLI_OUTPUT"
fi
