// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.4;

contract WinnerChallenge {
    event Winner(address);

    function attempt() external {
        require(msg.sender != tx.origin, "msg.sender is equal to tx.origin");
        emit Winner(msg.sender);
    }
}

contract AttackWinner {

  function attack(address _target) public {
      bytes memory payload = abi.encodeWithSignature("attempt()");
    (bool ok,) = _target.call(payload);
    require(ok);
  }
}