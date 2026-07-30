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

/// @notice Corrected counterpart to the intentionally vulnerable fixture.
/// @dev Signal layout for this fixture:
///      [0] authorized Merkle root
///      [1] scoped nullifier
///      [2] recipient encoded as uint160
///      [3] application domain commitment
contract SecureMembershipClaim {
    uint256 internal constant SNARK_SCALAR_FIELD =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;
    bytes32 internal constant ACTION_ID = keccak256("ZKBIND_MEMBERSHIP_CLAIM_V1");

    IGroth16Verifier public immutable verifier;
    mapping(uint256 root => bool authorized) public authorizedRoots;
    mapping(uint256 nullifier => bool used) public nullifierUsed;
    mapping(address recipient => bool claimed) public hasClaimed;

    error InvalidProof();
    error UnauthorizedRoot();
    error NullifierAlreadyUsed();
    error RecipientMismatch();
    error DomainMismatch();

    constructor(IGroth16Verifier verifier_, uint256 initialRoot) {
        verifier = verifier_;
        authorizedRoots[initialRoot] = true;
    }

    function claim(
        uint256[2] calldata a,
        uint256[2][2] calldata b,
        uint256[2] calldata c,
        uint256[4] calldata publicSignals,
        address recipient
    ) external {
        uint256 root = publicSignals[0];
        uint256 nullifier = publicSignals[1];
        uint256 provenRecipient = publicSignals[2];
        uint256 provenDomain = publicSignals[3];

        if (!authorizedRoots[root]) revert UnauthorizedRoot();
        if (nullifierUsed[nullifier]) revert NullifierAlreadyUsed();
        if (provenRecipient != uint256(uint160(recipient))) revert RecipientMismatch();
        if (provenDomain != applicationDomain()) revert DomainMismatch();
        if (!verifier.verifyProof(a, b, c, publicSignals)) revert InvalidProof();

        nullifierUsed[nullifier] = true;
        hasClaimed[recipient] = true;
    }

    function applicationDomain() public view returns (uint256) {
        return uint256(keccak256(abi.encode(block.chainid, address(this), ACTION_ID)))
            % SNARK_SCALAR_FIELD;
    }
}
