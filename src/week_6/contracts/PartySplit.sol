// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Party {
    uint256 public amount;
    mapping(address => bool) public participants;
    address[] participantsList;

    constructor(uint256 _amount) {
        amount = _amount;
    }

    function rsvp() external payable {
        require(msg.value == amount);
        require(!participants[msg.sender]);

        participants[msg.sender] = true;
        participantsList.push(msg.sender);
    }

    function payBill(address _venue, uint256 _billCost) external {
        (bool ok,) = _venue.call{value: _billCost}("");
        require(ok);

        uint256 refund = address(this).balance / participantsList.length;

        for (uint256 i = 0; i < participantsList.length; i++) {
            (ok,) = participantsList[i].call{value: refund}("");
            require(ok);
        }
    }
}
