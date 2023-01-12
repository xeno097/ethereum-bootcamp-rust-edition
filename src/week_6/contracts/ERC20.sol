// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Token {
    uint public totalSupply;
    string public name = "Name";
    string public symbol = "NME";
    uint8 public decimals = 18;

    mapping(address => uint256) public balanceOf;

    event Transfer(address from, address to, uint256 value);

    constructor() {
        totalSupply = 1000 * (10 ** decimals);
        balanceOf[msg.sender] = totalSupply;
    }

    function transfer(address _to, uint _value) external returns(bool){
        require(balanceOf[msg.sender] >= _value);

        balanceOf[msg.sender]-=_value;
        balanceOf[_to]+=_value;

        emit Transfer(msg.sender,_to,_value);

        return true;
    }

}