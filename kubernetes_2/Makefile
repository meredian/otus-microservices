DOCKER_SERVICE_TAG := meredian/k2:server

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


# .PHONY: build
# clone: clone_monstache

# .PHONY: build
# build: build_monstache build_plugin

# .PHONY: test
# test: test_plugin

# .PHONY: clone_monstache
# clone_monstache: clean
# 	git clone $(MONSTACHE_URL) $(MONSTACHE_FOLDER);
# 	cd $(MONSTACHE_FOLDER); git checkout tags/v5.7.4 -b build

# .PHONY: build_monstache
# build_monstache:
# 	cd $(MONSTACHE_FOLDER); go build

# .PHONY: clone_plugin
# clone_plugin:
# 	cp -r $(PLAIN_PLUGIN_FOLDER) $(MONSTACHE_FOLDER)/

# .PHONY: build_plugin
# build_plugin: clone_plugin
# 	cd $(MONSTACHE_FOLDER); \
# 	go build -buildmode=plugin -o \
# 		$(PLAIN_PLUGIN_FOLDER)/beameryTransform.so \
# 		$(PLAIN_PLUGIN_FOLDER)/beameryTransform.go

# .PHONY: test_plugin
# test_plugin: clone_plugin
# 	cd $(PLUGIN_FOLDER); \
# 	go test -v

# .PHONY: start
# start:
# 	$(MONSTACHE_BUILD) -mapper-plugin-path $(PLUGIN_BUILD) -f $(MONSTACHE_CONFIG)

# .PHONY: clean
# clean:
# 	rm -rf $(MONSTACHE_FOLDER)

# ## Setup e2e environment with BEAM. That Requires Mongo 4.0,
# # in case if you have old version of beam out there
# .PHONY: create_beam_env
# create_beam_env:
# 	beam service destroy mongo elasticsearch
# 	echo "Please do \"cd \$BEAM && rm -rf ./db/\" if you want to reset DB state"
# 	beam service start mongo elasticsearch
# 	sleep 20
# 	beam data run mgo-init-rs
# 	beam data run mgo reindex

