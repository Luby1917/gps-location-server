IMAGE_NAME=luby1917/geo-location-server
VERSION=`cat VERSION`
docker build -t IMAGE_NAME:VERSION && docker push IMAGE_NAME:VERSION