// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Collectible {
    event Deployed(address indexed owner);
    address public owner;
    uint price;

    event Transfer(address indexed from, address indexed to);

    event ForSale(uint price, uint timestamp);

    event Purchase(uint amount, address indexed buyer);

    constructor() {
        owner = msg.sender;
        emit Deployed(msg.sender);
    }

    function transfer(address to) external {
        require(msg.sender == owner);

        owner = to;
        emit Transfer(msg.sender, to);
    }

    function markPrice(uint _price) external {
        require(msg.sender == owner && _price != 0);
        price = _price;

        emit ForSale(_price, block.timestamp);
    }

    function purchase() external payable {
        require(msg.sender != owner);
        require(price != 0);
        require(msg.value == price);

        address oldOwner = owner;
        owner = msg.sender;
        price = 0;

        (bool ok,) = oldOwner.call{value: msg.value}("");
        require(ok);

        emit Purchase(msg.value,msg.sender);
    }
}