// SPDX-License-Identifier: GPL-3.0-only
pragma solidity ^0.8.20;

interface IVerifier {
    /// game (prover/verifier) name
    function name() external view returns (string memory);

    /// game task is permissioned or not
    function permission() external view returns (bool);

    // verify the zkp
    function verify(bytes calldata publics, bytes calldata proof) external view returns (bool);

    /// show how to serialize/deseriaze the inputs params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function inputs() external pure returns (string memory);

    /// show how to serialize/deserialize the publics params
    /// e.g. "uint256,bytes32,string,bytes32[],address[],ipfs"
    function publics() external pure returns (string memory);
}
