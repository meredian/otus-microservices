#/usr/bin/env bash
# helm upgrade -n monitoring --version "21.0.0" -f ./prometheus-values.yaml prometheus prometheus-community/kube-prometheus-stack

IS_INSTALLED=$(helm list -n monitoring | grep prometheus);
if [ -n "$IS_INSTALLED" ]; then
	echo 'Upgraging prometheus release with helm';
	helm upgrade -n monitoring --version "21.0.0" -f ./prometheus-values.yaml prometheus prometheus-community/kube-prometheus-stack
else \
	echo 'Installing prometheus release with helm';
	helm install -n monitoring --version "21.0.0" -f ./prometheus-values.yaml prometheus prometheus-community/kube-prometheus-stack
fi
