# Lapin Kubes

A demonstration of a peculiar networking issue when running Lapin + RabbitMQ
within a Kubernetes environment.

The code here is a simple adaptation of the Lapin minimal example merely
splitting it into two components, `client` and `server`, in order to have
more control over the requests being sent.

## Usage

Build as a Docker container and deploy into a Kubernetes cluster. An
example configuration YAML file is supplied in `k8s/lapin-kubes.yml`.

## Issue

Under Kubernetes the `server` stays connected and runs, producing regular
heartbeats, but will not actually process any messages after ~10s of idle
activity, or around ~40s after starting regardless of activity.

This does not happen in Docker on the same host/OS. It does not happen on
the host operating system.

This does happen within Kubernetes to both Kubernetes-hosted RabbitMQ, and
an external (non-Kubernetes) RabbitMQ server.

> Tested using Kubernetes v1.22.1 on Docker 20.10.8 using Flannel as a CNI.

See also:

* [Issue #347 - Kubernetes](https://github.com/CleverCloud/lapin/issues/347)
* [Issue #348 - Delivery stalls](https://github.com/CleverCloud/lapin/issues/348)
