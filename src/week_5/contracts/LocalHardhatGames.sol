//SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Game1 {
  event Winner(address winner);

  function win() public {
    emit Winner(msg.sender);
  }
}

contract Game2 {
  uint public x;
  uint public y;

  function setX(uint _x) external {
    x = _x;
  }

  function setY(uint _y) external {
    y = _y;
  }

  event Winner(address winner);

  function win() public {
    require(x > 0 && y > 0);
    require(x + y == 50);
    emit Winner(msg.sender);
  }
}

contract Game3 {
  uint8 y = 210;

  event Winner(address winner);

  function win(uint8 x) public {
    unchecked {
        uint8 sum = x + y;
        require(sum == 255, "Incorrect argument passed in!");
    }
    emit Winner(msg.sender);
  }
}

contract Game4 {
  uint8 y = 210;

  event Winner(address winner);

  function win(uint8 x) public {
    unchecked {
        uint8 sum = x + y;
        require(sum == 10, "Incorrect argument passed in!");
    }
    emit Winner(msg.sender);
  }
}

contract Game5 {
  mapping(address => uint) balances;
  mapping(address => uint) allowances;

  function giveMeAllowance(uint allowance) external {
    allowances[msg.sender] += allowance;
  }

  function mint(uint amount) external {
    allowances[msg.sender] -= amount;
    balances[msg.sender] += amount;
  }

  event Winner(address winner);

  function win() public {
    require(balances[msg.sender] >= 10000);
    emit Winner(msg.sender);
  }
}