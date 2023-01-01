//SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Game1 {
    event Winner(address winner);

    function win() public {
        emit Winner(msg.sender);
    }
}

contract Game2 {
    uint256 public x;
    uint256 public y;

    function setX(uint256 _x) external {
        x = _x;
    }

    function setY(uint256 _y) external {
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
    mapping(address => uint256) balances;
    mapping(address => uint256) allowances;

    function giveMeAllowance(uint256 allowance) external {
        allowances[msg.sender] += allowance;
    }

    function mint(uint256 amount) external {
        allowances[msg.sender] -= amount;
        balances[msg.sender] += amount;
    }

    event Winner(address winner);

    function win() public {
        require(balances[msg.sender] >= 10000);
        emit Winner(msg.sender);
    }
}
