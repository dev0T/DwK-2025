# DummySite CRD

This exercise is folder has the following structure:
- `controller` contains the controller for the `DummySite` CRD along with its definitions in the `controller/manifests` folder.
- `dummysite` contains the application that will be run by the `DummySite` CRD when it is created.

## Getting Started

To be enable to create `DummySites`, from the `dummy` folder run:

```kubectl apply -k controller/manifests```

After that just apply the desired DummySite. To use the one used for testing, from the `dummy` folder run:


```kubectl apply -f dummysite/manifests```

## How it works

The controller detects when events related to `DummySites` happen using the k8s API. Depending on these events, it will take different actions such as creating a Pod, deleting or modifying it. Functonality was implemented following examples provided by `kube` and `k8s-openapi` crates.

Creating a reflector would most likely be the preferable approach, but for this exercise it wasn't been done due to time restraints.