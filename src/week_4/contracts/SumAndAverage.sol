// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract SumAndAverage {
    function sumAndAverage(uint256 a, uint256 b, uint256 c, uint256 d) external pure returns (uint256, uint256) {
        uint256 sum = a + b + c + d;
        return (sum, sum / 4);
    }
}
