// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Countdown {
    uint256 ticker = 10;
    address owner;

    constructor(){
        owner = msg.sender;
    }

    function tick() external {
        ticker--;
        if(ticker == 0){
            selfdestruct(payable(owner));
        }
    }
}