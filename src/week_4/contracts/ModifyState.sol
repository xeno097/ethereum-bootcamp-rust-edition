//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.4;

contract ModifyState {
  uint public x;

  constructor(uint _x) {
    x = _x;
  }

  function modifyToLeet() public {
    x = 1337;
  }

}