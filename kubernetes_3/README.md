kubernetes_3
============

Folder includes:
* Application (in Rust) for managing `/user/` route & with `/health` handler
* Helm package to install PostgreSQL
* Kubernetes manifest to deploy application as a service with Nginx ingress.
* Postman collection for all handlers

Service is exposed as `http://arch.homework/`.

## Quickstart

* Check that you have prerequisites installed
* For full development cycle user `make` (But you need dockerhub access to succeed)
* To install & test run `make helm apply restart wait newman logs`

## Prerequisites

* Rust (just pick latest) for local dev
* `docker` for building service image
* `kubectl` for deploying k8s manifests
* `kubetail` for logs
* `helm` for installing HELM dependencies
* `newman` for testing Postman collections from CLI
* Some available kubernetes env (e.g. `minikube` or cloud)

## Usage

* `make build` to build docker image of Rust service
* `make push` to push image to Docker Hub (if you're `meredian` user :wink:)
* `make helm` to install HEML dependencies
* `make apply` to apply manifests
* `make restart` to restart pods (in case image was changed, but not manifest change for deployment)
* `make wait` to wait for service to spin up (helpful to wait for helm installation to succeed)
* `make newman` to quickly check API with `newman`
* `make logs` to watch logs on deployed pods
* `make` will trigger whole sequence in that order

