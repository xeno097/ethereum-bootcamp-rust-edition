// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Escrow {
    address public depositor;
    address public beneficiary;
    address public arbiter;
    bool public isApproved = false;

    event Approved(uint balance);

    constructor(address _arbiter, address _beneficiary) payable {
        depositor = msg.sender;
        arbiter = _arbiter;
        beneficiary = _beneficiary;
    }


    function approve() external {
        require(msg.sender == arbiter);

        uint balance =address(this).balance;

        isApproved = true;
        (bool ok,) = beneficiary.call{value: balance}("");
        require(ok);

        emit Approved(balance);
    }
}