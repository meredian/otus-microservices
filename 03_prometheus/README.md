kubernetes_3
============

Folder includes:
* Application (in Rust) for managing `/user/` route & with `/health` handler
* Helm package containing our application with all dependencies added
* Postman collection for all handlers
* Helper scripts

Service is exposed as `http://arch.homework/`.

## Quickstart

* Check that you have prerequisites installed
* To deploy & test run `make`
* For full development cycle user `make dev` (But you need dockerhub access to succeed)

## Prerequisites

* Rust (just pick latest) for local dev
* `docker` for building service image
* `kubectl` for deploying k8s manifests (check `make` for exact version)
* `kubetail` for logs
* `helm` for installing HELM dependencies (`v3.7.1`)
* `newman` for testing Postman collections from CLI
* Some available kubernetes env (e.g. `minikube` or cloud)

## Usage

### If you're using minikube
To enable `http://arch.homework`, you'll need add record to `/etc/hosts` with
proper ip obtained by `minikube ip`. e.g:
```
192.168.1.23 arch.homework
```

### What can we make?

* `make minikube` - spin up Minikube instance & enable ingress.
* `make build` to build docker image of Rust service
* `make push` to push image to Docker Hub (if you're `meredian` user :wink:)
* `make deploy` to install our app with dependencies (helm-packed)
* `make restart` to restart pods (in case image was changed, but not manifest change for deployment)
* `make wait` to wait for service to spin up (helpful to wait for helm installation to succeed)
* `make newman` to quickly check API with `newman`
* `make logs` to watch logs on deployed pods
* `make` to deploy & test sequence
* `make dev` will build, push image & run `make` subsequently

