// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract SumAndAverage {
    
    function sumAndAverage(uint a,uint b,uint c, uint d) external pure returns(uint,uint){
        uint sum = a + b + c +d;
        return (sum, sum / 4);
    }
}