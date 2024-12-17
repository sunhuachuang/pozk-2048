// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IVerifier {
    /// game (prover/verifier) name
    function name() external view returns (string memory);

    /// check task sender is permissioned or not
    function permission(address sender) external view returns (bool);

    // verify the zkp
    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool);

    /// show how to serialize/deseriaze the inputs params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function inputs() external pure returns (string memory);

    /// show how to serialize/deserialize the publics params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function publics() external pure returns (string memory);

    /// show the prover supported types
    /// e.g. "zk", "risc0,sp1", "candle,ollama"
    function types() external pure returns (string memory);
}
