//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.4;

contract ModifyState {
    uint256 public x;

    constructor(uint256 _x) {
        x = _x;
    }

    function modifyToLeet() public {
        x = 1337;
    }
}
