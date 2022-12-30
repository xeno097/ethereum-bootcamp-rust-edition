// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Setter {
    uint256 public value;

    function modify(uint256 _value) external {
        value = _value;
    }
}
