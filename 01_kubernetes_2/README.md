kubernetes_2
============

Folder includes:
* Simple application (in Rust) which responses 200 OK to `GET /` and `GET /health`
* Kubernetes manifest to deploy it as a service with Nginx ingress. Service is exposed as `http://arch.homework/`.

## Prerequisites

* Rust (just pick latest) for local dev
* `docker` for building service image
* `kubectl` for deploying k8s manifests
* `kubetail` for logs
* Some available kubernetes env (e.g. `minikube` or cloud)

## Usage

* `make build` to build docker image of rust service
* `make push` to push image to Docker Hub (if you're `meredian` user :wink:)
* `make apply` to apply manifests
* `make restart` to restart pods (in case image was changed, but not manifest change for deployment)
* `make test` to run cURL queries to all expected urls, quick check
* `make logs` to watch logs on deployed pods
* `make` will trigger whole sequence in that order
