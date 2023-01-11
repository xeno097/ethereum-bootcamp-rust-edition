// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Switch {
    
    address recipient;
    address owner;
    uint256 constant period = 52 weeks;
    uint256 pingLimit;

    constructor(address _recipient) payable {
        recipient = _recipient;
        owner = msg.sender;
        pingLimit = block.timestamp + period;
    }

    function withdraw() external {
        require(msg.sender == recipient && block.timestamp > pingLimit);

        (bool ok,) = recipient.call{value: address(this).balance}("");
        require(ok);
    }

    function ping() external {
        require(msg.sender == owner);
        pingLimit = block.timestamp + period;
    }
}