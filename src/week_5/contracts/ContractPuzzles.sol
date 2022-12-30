// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

contract Game1 {
  bool public isWon;
  bool public unlocked;

  function unlock() external {
    unlocked = true;
  }

  function win() external {
    require(unlocked, "Nope. Try again!");

    isWon = true;
  }
}

contract Game2 {
  bool public isWon;
  mapping(uint => bool) switches;

  function switchOn(uint key) payable external {
    switches[key] = true;
  }

  function win() external {
    require(switches[20]);
    require(switches[47]);
    require(switches[212]);

    isWon = true;
  }
}

contract Game3 {
  bool public isWon;
  mapping(address => uint) balances;

  function buy() payable external {
    balances[msg.sender] += msg.value;
  }

  function win(address addr1, address addr2, address addr3) external {
    require(balances[addr3] > 0);
    require(balances[addr2] > balances[addr1]);
    require(balances[addr1] > balances[addr3]);

    isWon = true;
  }
}

contract Game4 {
  bool public isWon;

  mapping(address => mapping(address => bool)) nested;

  function write(address x) external {
    nested[x][msg.sender] = true;
  }

  function win(address y) external {
    require(nested[msg.sender][y], "Nope. Try again!");

    isWon = true;
  }
}

contract Game5 {
  bool public isWon;

  address threshold = 0x00FfFFfFFFfFFFFFfFfFfffFFFfffFfFffFfFFFf;

  function win() external {
    require(bytes20(msg.sender) < bytes20(threshold), "Nope. Try again!");

    isWon = true;
  }
}