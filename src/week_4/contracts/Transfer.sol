// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Transfer {
    mapping(address => uint) public balances;

    constructor() {
        balances[msg.sender] = 1000;
    }

    function transfer(address beneficiary, uint amount) external {
        require(balances[msg.sender] >= amount, "Balance too low!");
        balances[beneficiary] += amount;
        balances[msg.sender] -= amount;
    }
}