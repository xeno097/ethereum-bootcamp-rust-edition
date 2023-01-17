// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

library UIntFunctions {
    

    function isEven(uint number) external pure returns(bool){
        return number % 2 == 0;
    }
}

contract Game {
    using UIntFunctions for uint;
    uint public participants;
    bool public allowTeams;

    constructor(uint _participants){
        participants = _participants;
        allowTeams = participants.isEven();
    }



}