# Only use these programs directly or explain yourself:
#  awk cat cmp cp diff echo egrep expr false grep install-info ln ls
#  mkdir mv printf pwd rm rmdir sed sleep sort tar test touch tr true

DOCKER_IMAGE=tetani_example

.PHONY: docker-build
all: docker-build

docker-build:
	docker build --tag=${DOCKER_IMAGE} .
