# Carver, LSP-based reducer for compiled languages

Carver leverages the state tracked by LSP servers to target and reduce code across dependencies and workspaces.

## Steps

- [ ] connect to rust-analyzer
- [ ] communicate with rust-analyzer to load a local crate

- Vendors dependencies to have a local, editable copy within the repository.
- Edits relevant documents to refer to the locally vendored copies, ex. `Cargo.toml`
- Uses a collection of Chisels (reduction methods) to methodically reduce the source code
- Utiliizes Chisels in randomized or systematic patterns to prevent stagnation at local minimas.
- Launches and connects to LSP servers, ex. `rust-analyzer`
- Uses parsers (regex?) to identify the spans of code to be modified
- Queries the LSP with said spans to identify function definiton and all references, MarkingPoints.
- Outlines ChiselTargets (spans) to be modified relative to Points
- Checks for crossover of ChiselTargets

1. Select Chisel
2. Mark Points
3. Get ChiselTargets (scopes) relevant to Points
4. Carve
5. Check if error is the same as before, if not, cache the error and backstep

# References

- [LSP-proxy](https://github.com/tfpk/lsp-proxy/blob/main/src/proxy.rs) by tfpk
