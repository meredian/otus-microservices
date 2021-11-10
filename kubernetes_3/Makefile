DOCKER_SERVICE_TAG := meredian/otus-microservices:k3-service

SERVICE_FOLDER := service
K8S_FOLDER := kubernetes
CHARTS_FOLDER := charts
POSTMAN_FOLDER := postman
SCRIPTS_FOLDER := scripts

ROOT_URI := http://arch.homework

all: build push helm apply restart wait newman logs

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
	@IS_HELM_INSTALLED=$$(helm list -n meredian | grep postgres); \
	if [ -n "$$IS_HELM_INSTALLED" ]; then \
		echo 'Postgres is already installed from helm, skipping...'; \
	else \
		echo 'Installing postgres from helm, skipping...'; \
		cd $(CHARTS_FOLDER) && helm install postgres \
			-n meredian --create-namespace \
			-f ./bitnami-postgresql.values.yaml \
			./bitnami-postgresql-10.13.7.tgz; \
	fi

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


.PHONE: wait
wait:
	$(SCRIPTS_FOLDER)/wait_with_curl.sh

.PHONY: newman
newman:
	cd $(POSTMAN_FOLDER) && \
	newman run arch.homework-k3.postman_collection.json \
		-e local.postman_environment.json --verbose

.PHONY: logs
logs:
	kubetail -f k3 -n meredian
