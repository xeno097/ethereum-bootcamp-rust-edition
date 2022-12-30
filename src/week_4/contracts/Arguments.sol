// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Arguments {
    uint256 public x;

    constructor(uint256 _x) {
        x = _x;
    }

    function increment() external {
        x += 1;
    }

    function add(uint256 num) external view returns (uint256) {
        return x + num;
    }
}
