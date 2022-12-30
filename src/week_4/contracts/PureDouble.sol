// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract PureDouble {
    function double(uint256 num) external pure returns (uint256) {
        return num * 2;
    }

    function double(uint256 num1, uint256 num2) external pure returns (uint256, uint256) {
        return (num1 * 2, num2 * 2);
    }
}
