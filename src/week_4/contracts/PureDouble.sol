// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract PureDouble {
    function double(uint num) external pure returns(uint){
        return num * 2;
    }

    function double(uint num1, uint num2) external pure returns(uint,uint){
        return (num1*2,num2*2);
    }
}