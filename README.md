# Transactional

Inspired by Spring Transactional

Since Rust does not have the concept of runtime reflection, it is implemented by performing code generation through macros at compile time.

Implemented using proc macros due to their broader control scope.

Proc macros currently need to be separated into their own modules.

Therefore, it is composed of multiple crates, and as more modules will be added in the future, descriptions for each module will be included.

---

# Crates

## Proc Macro

- `transactional`

Inspired by Spring Transactional

## Example Crates

In addition to implementing the `Transactional` proc macro, various example crates are included.

Examples showing how verbose the code can become without using proc macros.

- `pure_transaction_ex`

## Submodules

- `rombok`

- `shared`

`shared` has not been included as a submodule yet.

It is a module created to avoid writing common code repeatedly for multiple examples.
