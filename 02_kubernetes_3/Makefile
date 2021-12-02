KUBERNETES_VERSION := v1.22.3
DOCKER_SERVICE_TAG := meredian/otus-microservices:k3-service

SERVICE_FOLDER := service
CHART_FOLDER := myapp-chart
POSTMAN_FOLDER := postman
SCRIPTS_FOLDER := scripts

ROOT_URI := http://arch.homework

all: deploy restart wait newman logs
dev: build push all

.PHONY: minikube
minikube:
	@# --vm-driver=hyperkit: Docker driver is used by default, provides bad network
	@# --cni=flannel: Docker bridge is used by default, also not nice.
	minikube start \
	--vm-driver=hyperkit \
	--cni=flannel \
	--cpus=2 --memory=8g \
	--kubernetes-version=$(KUBERNETES_VERSION) \
	&& minikube addons enable ingress

.PHONY: build
build:
	cd $(SERVICE_FOLDER) && docker build . -t $(DOCKER_SERVICE_TAG)

.PHONY: push
push:
	docker push $(DOCKER_SERVICE_TAG)

.PHONY: deploy
deploy:
	@IS_INSTALLED=$$(helm list -n meredian | grep myapp); \
	if [ -n "$$IS_INSTALLED" ]; then \
		echo 'Upgraging release from helm'; \
		helm upgrade myapp ./$(CHART_FOLDER) -n meredian; \
	else \
		echo 'Installing release from helm'; \
		helm install myapp ./$(CHART_FOLDER) -n meredian --create-namespace; \
	fi

.PHONY: uninstall
uninstall:
	helm uninstall myapp -n meredian

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
	kubetail -f myapp -n meredian
