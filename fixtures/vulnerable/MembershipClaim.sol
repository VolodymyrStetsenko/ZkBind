// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.24;

interface IGroth16Verifier {
    function verifyProof(
        uint256[2] calldata a,
        uint256[2][2] calldata b,
        uint256[2] calldata c,
        uint256[4] calldata publicSignals
    ) external view returns (bool);
}

/// @notice Intentionally vulnerable fixture used to test ZKBind rules.
/// @dev The verified statement is not bound to the recipient, chain, contract,
///      protected action, or a persisted nullifier.
contract VulnerableMembershipClaim {
    IGroth16Verifier public immutable verifier;
    mapping(address recipient => bool claimed) public hasClaimed;

    constructor(IGroth16Verifier verifier_) {
        verifier = verifier_;
    }

    function claim(
        uint256[2] calldata a,
        uint256[2][2] calldata b,
        uint256[2] calldata c,
        uint256[4] calldata publicSignals,
        address recipient
    ) external {
        require(verifier.verifyProof(a, b, c, publicSignals), "invalid proof");

        // A copied proof can be submitted with an attacker-selected recipient.
        // The contract also has no proof-level replay protection.
        hasClaimed[recipient] = true;
    }
}
