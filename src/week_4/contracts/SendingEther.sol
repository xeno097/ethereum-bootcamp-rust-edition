// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract SendingEther {
    address public owner;
    address public charity;

    constructor(address _charity) {
        owner = msg.sender;
        charity = _charity;
    }

    receive() external payable{}

    function tip() external payable {
        (bool ok,) = owner.call{value: msg.value}("");
        require(ok);
    }

    function donate() external {
        selfdestruct(payable(charity));
    }
}