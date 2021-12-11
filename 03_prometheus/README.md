prometheus
==========

Focus is to add Prometheus instrumentation, exporters & Grafana dashboards to
represent gathered data.

Folder includes:
* Application (in Rust) from previous lesson, instrumented with Prometheus exporter
* Folder with global operators to devide CRDs (Custom Resource definitions)
* Helm package containing our application with all dependencies added
* Postman collection for all handlers
* Helper scripts
* Example image how should result look like

Service is exposed as `http://arch.homework/`
Grafana is exposed as `http://grafana.arch.homework/`
Prometheus is exposed as `http://prometheus.arch.homework/`

## Quickstart

* Check that you have prerequisites installed
* Start minikube with `make minikube` if you're starting fresh
* Install required operators for deploying CRD with `make operators` (Only should be done once)
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
* `make operators` - install required K8S Operators with CRDs
* `make build` to build docker image of Rust service
* `make push` to push image to Docker Hub (if you're `meredian` user :wink:)
* `make deploy` to install our app with dependencies (helm-packed)
* `make uninstall` to uninstall app from cluster
* `make restart` to restart pods (in case image was changed, but not manifest change for deployment)
* `make wait` to wait for service to spin up (helpful to wait for helm installation to succeed)
* `make newman` to quickly check API with `newman`
* `make logs` to watch logs on deployed pods
* `make` to deploy & test sequence
* `make dev` will build, push image & run `make` subsequently

