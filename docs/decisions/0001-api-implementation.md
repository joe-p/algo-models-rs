---
# These are optional metadata elements. Feel free to remove any of them.
status: proposed
date: 2024-12-19
decision-makers: Michael Feher, Bruno Martins, Joe Polny, David Rojas
consulted: Algorand Foundation CTO office, MakerX engineering team
---

# API Client Implementation

## Context and Problem Statement

A core part of interacting with the Algorand network is using the APIs provided by `algod`. The endpoints are used for crucial actions such as sending transactions and reading active state. There are also developer-focused APIs such as disassembly and simulate. A comprehensive suite of libraries for Algorand must include a way to interact with nodes via the HTTP API.

## Decision Drivers

- Should have a low maintenance cost
- Should have cross-language consistency
- Should allow control of HTTP requests and responses

## Considered Options

- Language-specific [OpenAPI Generator bindings](https://github.com/OpenAPITools/openapi-generator)
- Rust implementation for making HTTP calls with FFI bindings
- Rust implementation for constructing/parsing HTTP requests/responses with FFI bindings

## Decision Outcome

TBD

### Confirmation

The decided implementation should have an initial spike that makes use of various critical and developer-related APIs

## Pros and Cons of the Options

### Language-specific OpenAPI Generator bindings

<!-- This is an optional element. Feel free to remove. -->

- Good, because it supports a huge list of languages/clients
- Good, because there is essential zero maintenance cost (assuming well-maintained OAPI spec)
- Neutral, because it will require better OAPI specs upstream in `go-algorand`
- Bad, because it doesn't give us control over the interfaces meaning we won't be able to provide useful abstractions
- Bad, because it might not integrate seamlessly with other core modules

### Rust implementation for making HTTP calls with FFI bindings

- Good, because we have direct control over the interfaces and can provide useful abstractions
- Good, because we can ensure it integrates well with other core modules
- Bad, because of the FFI overhead cost
- Bad, because it might be difficult to provide low-level control of HTTP requests and responses
- Bad, because it might not enable idiomatic HTTP requests

### Rust implementation for constructing/parsing HTTP requests/responses with FFI bindings

- Good, because we have direct control over the interfaces and can provide useful abstractions
- Good, because we can ensure it integrates well with other core modules
- Good, because it allows clients to use whichever HTTP client they'd like
- Bad, because of the FFI overhead cost

## More Information

This ADR is specifically focused on `algod` APIs, but the same strategy can and will likely be used for KMD and Indexer APIs as well.
