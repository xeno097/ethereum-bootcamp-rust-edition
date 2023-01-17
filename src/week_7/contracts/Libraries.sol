// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

library UIntFunctions {
    function isEven(uint256 number) external pure returns (bool) {
        return number % 2 == 0;
    }
}

contract Game {
    using UIntFunctions for uint256;

    uint256 public participants;
    bool public allowTeams;

    constructor(uint256 _participants) {
        participants = _participants;
        allowTeams = participants.isEven();
    }
}

library Prime {
    function dividesEvenly(uint256 num1, uint256 num2) external pure returns (bool) {
        return num1 % num2 == 0;
    }

    function isPrime(uint256 num) external pure returns (bool) {
        for (uint256 i = 2; i <= num / 2; i++) {
            if (num % i == 0) {
                return false;
            }
        }

        return true;
    }
}

contract PrimeGame {
    using Prime for uint256;

    function isWinner() external view returns (bool) {
        return block.number.isPrime();
    }
}
