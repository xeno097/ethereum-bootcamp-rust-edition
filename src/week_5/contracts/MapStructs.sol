// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

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
