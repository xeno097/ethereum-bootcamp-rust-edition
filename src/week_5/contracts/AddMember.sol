// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract AddMember {
    mapping(address => bool) public members;

    function addMember(address _newMember) external {
        members[_newMember] = true;
    }

    function isMember(address _member) external view returns(bool){
        return members[_member];
    }

    function removeMember(address _member) external {
        members[_member] = false;
    }
}