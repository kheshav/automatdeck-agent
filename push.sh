#!/bin/bash

if [[ "$CI_COMMIT_BRANCH" == "master" ]]; then
    mytarget="uat"
fi

if [[ "$CI_COMMIT_BRANCH" == "master" && "$CI_COMMIT_TAG" == null ]]; then
    mytarget="prod"
fi

url="http://api.automatdeck-${mytarget}.svc.cluster.local:8000/62268f76fff54aea8aac44a0fd9a28c9/"

cd build

for FILE in **/*
do
    echo "Uploading file $FILE"
    set -- "$FILE" 
    IFS="/"; declare -a Array=($*)
    echo "Arch: ${Array[0]}" 
    echo "File: ${Array[1]}"
    _md5sum=`md5sum ${Array[0]}/${Array[1]} | awk '{print $1}'`
    _sha256=`sha256sum ${Array[0]}/${Array[1]} | awk '{print $1}'`
    curl -F "md5sum=$_md5sum" -F "sha256=$_sha256" -F "name=${Array[1]}" -F "package=@$FILE" -H "access_key:$repo_access_key" -H "secret_key:$repo_secret_key" "$url" -vv
    curl -F "md5sum=$_md5sum" -F "sha256=$_sha256" -F "name=ad-agent_latest_${Array[0]}.tar.gz" -F "package=@$FILE" -H "access_key:$repo_access_key" -H "secret_key:$repo_secret_key" "$url" -vv
done    
