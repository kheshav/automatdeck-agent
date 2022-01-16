#!/bin/sh

if [[ "$CI_COMMIT_BRANCH" == "master" ]]; then
    environment="uat"
fi

if [[ "$CI_COMMIT_BRANCH" == "master" && "$CI_COMMIT_TAG" == null ]]; then
    url="prod"
fi

url="http://api.automatdeck-${environment}.svc.cluster.local:8000/62268f76fff54aea8aac44a0fd9a28c9/"

cd build
for FILE in *
do
    echo "Uploading file $FILE"
    curl -F "name=$FILE" -F "package=@$FILE" -H "access_key:$repo_access_key" -h "secret_key:$repo_secret_key" $url
done
