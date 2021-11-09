DOCKER_SERVICE_TAG := meredian/otus-microservices:k3-service

SERVICE_FOLDER := service
K8S_FOLDER := kubernetes
CHARTS_FOLDER := charts

ROOT_URI := http://arch.homework

all: build push helm apply restart test logs

.PHONY: minikube
minikube:
	minikube start --vm-driver=hyperkit && minikube addons enable ingress

.PHONY: build
build:
	cd $(SERVICE_FOLDER) && docker build . -t $(DOCKER_SERVICE_TAG)

.PHONY: push
push:
	docker push $(DOCKER_SERVICE_TAG)

.PHONY: helm
helm:
	@IS_HELM_INSTALLED=$$(helm list -n meredian | grep postgres) && \
    if [ -n "$$IS_HELM_INSTALLED" ]; then \
		echo 'Postgres already installed from helm, skipping...'; \
    else \
		echo 'Installing postgres from helm' && \
		cd $(CHARTS_FOLDER) && helm install postgres \
			-n meredian --create-namespace \
			-f ./bitnami-postgresql.values.yaml \
			./bitnami-postgresql-10.13.7.tgz; \
    fi

# 	@IS_HELM_INSTALLED=$$(helm list -n meredian | grep postgres) && \
# 	echo '$@: IS_HELM_INSTALLED is $$IS_HELM_INSTALLED' && \
# 	if [ -n "$$IS_HELM_INSTALLED" ]; then \
# 		echo '$@: Postgres already installed from helm, skipping' \
# 	fi && \
# 	echo '$@: done'

.PHONY: apply
apply:
	kubectl apply -f $(K8S_FOLDER)/

.PHONY: k8s
k8s: helm apply

# Handy way to update image if you have a new image & "Always"
# image pull policy by gracefull restart of all k8s deployments
.PHONY: restart
restart:
	kubectl rollout restart deployment -n meredian

.PHONY: logs
logs:
	kubetail -f k3 -n meredian

.PHONY: test
test:
	@echo "\nShould return 200 OK \"Hello world!\" on GET ${ROOT_URI} URL"
	curl ${ROOT_URI} -v
	@echo "\n\nShould return 200 OK {\"status\":\"OK\", \"DB\":\"OK\"} on GET ${ROOT_URI}/health URL"
	curl ${ROOT_URI}/health -v
	@echo "\n\nShould return 200 OK \"Hello world!\" on GET ${ROOT_URI}/meredian/some-url URL (rewrite URL to /)"
	curl ${ROOT_URI}/otusapp/meredian/some-url -v
