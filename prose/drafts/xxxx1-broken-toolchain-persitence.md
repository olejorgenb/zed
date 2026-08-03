Ref PR #19576 which introduced this. !

In short - the selected toolchain is persisted using the `WorkTreeId` as a database key. This ID is not stable so the persistence does not work and tool-chain rows accumulate in the `toolchains` sqlite table.
