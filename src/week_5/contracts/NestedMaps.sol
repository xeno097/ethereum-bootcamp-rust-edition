// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract NestedMaps {
    enum ConnectionTypes {
        Unacquainted,
        Friend,
        Family
    }

    mapping(address => mapping(address => ConnectionTypes)) public connections;

    function connectWith(address other, ConnectionTypes connectionType) external {
        connections[msg.sender][other] = connectionType;
    }
}
