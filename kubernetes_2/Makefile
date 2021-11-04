DOCKER_SERVICE_TAG := meredian/otus-microservices:k2-service

SERVICE_FOLDER := service
K8S_FOLDER := kubernetes

ROOT_URI := http://arch.homework

all: build push apply restart test logs

.PHONY: build
build:
	cd $(SERVICE_FOLDER) && docker build . -t $(DOCKER_SERVICE_TAG)

.PHONY: push
push:
	docker push $(DOCKER_SERVICE_TAG)

.PHONY: apply
apply:
	kubectl apply -f $(K8S_FOLDER)/

# Handy way to update image if you have a new image & "Always"
# image pull policy by gracefull restart of all k8s deployments
.PHONY: restart
restart:
	kubectl rollout restart deployment -n meredian

.PHONY: logs
logs:
	kubetail -f k2 -n meredian

.PHONY: test
test:
	@echo "\nShould return 200 OK \"Hello world!\" on GET ${ROOT_URI} URL"
	curl ${ROOT_URI} -v
	@echo "\n\nShould return 200 OK {\"status\":\"ok\"} on GET ${ROOT_URI}/health URL"
	curl ${ROOT_URI}/health -v
	@echo "\n\nShould return 200 OK \"Hello world!\" on GET ${ROOT_URI}/meredian/some-url URL (rewrite URL to /)"
	curl ${ROOT_URI}/otusapp/meredian/some-url -v
