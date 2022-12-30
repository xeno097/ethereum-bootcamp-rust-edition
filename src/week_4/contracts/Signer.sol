// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Signer {
    address owner;
    string public message;

    constructor() {
        owner = msg.sender;
    }

    function modify(string calldata _message) external {
        require(msg.sender != owner, "Owner cannot modify the message!");
        message = _message;
    }
}
