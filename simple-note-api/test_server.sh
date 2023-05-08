#!/bin/bash  

# create variable containing url path to server, get, post, put, delete
url="localhost:8080/notes"

echo "Testing server at $url"

# create 100 post requests using curl and random json data from faker
echo "Beginning 100 post requests"

for i in {1..100}
do
  curl -X POST -H "Content-Type: application/json" -d '{"title":"'"Address of $(faker name)"'","body":"'"$(faker address)"'"}' $url
done

echo "Finished 100 post requests"


# create get request of all notes and save the id to a temporary file in linux temp directory
curl -X GET $url > /tmp/notes.txt

# create 100 get requests using curl with id from temp file
for i in {1..100}
do
  curl -X GET $url/$(cat /tmp/notes.txt | jq -r '.['$i-1']._id')
done

# create 100 put requests using curl and random json data from faker with id from temp file
for i in {1..100}
do
  curl -X PUT -H "Content-Type: application/json" -d '{"title":"'"Address of $(faker name)"'","body":"'"$(faker address)"'"}' $url/$(cat /tmp/notes.txt | jq -r '.['$i-1']._id')
done

# create 100 delete requests using curl and random json data from faker with id from temp file
for i in {1..100}
do
  curl -X DELETE $url/$(cat /tmp/notes.txt | jq -r '.['$i-1']._id')
done

# delete temp file
rm /tmp/notes.txt

# get all
curl -X GET $url
