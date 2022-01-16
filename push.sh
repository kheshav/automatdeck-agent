#!/bin/sh

# Based on Environment set URL
if [[ "$CI_COMMIT_BRANCH" == "master" ]]; then
    environment="uat"
fi

if [[ "$CI_COMMIT_BRANCH" == "master" && "$CI_COMMIT_TAG" == null ]]; then
    url="prod"
fi

url="http://api.automatdeck-${environment}.svc.cluster.local:8000/62268f76fff54aea8aac44a0fd9a28c9/"

for FILE in **/*
do
    echo "Uploading file $FILE"
    set -- "$FILE" 
    IFS="/"; declare -a Array=($*)
    echo "Arch: ${Array[0]}" 
    echo "File: ${Array[1]}"
    curl -F "name=${Array[1]}" -F "package=@$FILE" -H "access_key:$repo_access_key" -H "secret_key:$repo_secret_key" $url
    curl -F "name=ad-agent_latest_${Array[0]}.tar.gz" -F "package=@$FILE" -H "access_key:$repo_access_key" -H "secret_key:$repo_secret_key" $url
done    
