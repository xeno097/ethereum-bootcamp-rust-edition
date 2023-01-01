// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract AddMember {
    mapping(address => bool) public members;

    function addMember(address _newMember) external {
        members[_newMember] = true;
    }

    function isMember(address _member) external view returns (bool) {
        return members[_member];
    }

    function removeMember(address _member) external {
        members[_member] = false;
    }
}

contract MapStructs {
    struct User {
        uint256 balance;
        bool isActive;
    }

    mapping(address => User) public users;

    function createUser() external {
        require(!users[msg.sender].isActive);

        users[msg.sender].isActive = true;
        users[msg.sender].balance = 100;
    }

    function transfer(address to, uint256 amount) external {
        require(users[msg.sender].isActive && users[to].isActive);
        require(users[msg.sender].balance >= amount);

        users[msg.sender].balance -= amount;
        users[to].balance += amount;
    }
}

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
