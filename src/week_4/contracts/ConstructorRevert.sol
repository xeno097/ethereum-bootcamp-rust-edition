// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract ConstructorRevert {
    address owner;

    constructor() payable {
        require(msg.value >= 1 ether);
        owner = msg.sender;
    }

    function withdraw() public {
        require(msg.sender == owner);

        (bool ok,) = owner.call{value: address(this).balance}("");
        require(ok);
    }
}