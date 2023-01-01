// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract FixedSum {
    function sum(uint256[5] memory numbers) external pure returns (uint256) {
        uint256 res = 0;

        for (uint256 i = 0; i < numbers.length; i++) {
            res += numbers[i];
        }

        return res;
    }
}

contract DynamicSum {
    function sum(uint256[] memory numbers) external pure returns (uint256) {
        uint256 res = 0;
        for (uint256 i = 0; i < numbers.length; i++) {
            res += numbers[i];
        }

        return res;
    }
}

contract FilterToStorage {
    uint256[] public evenNumbers;

    function filterEven(uint256[] memory numbers) external {
        for (uint256 i = 0; i < numbers.length; i++) {
            if (numbers[i] % 2 == 0) {
                evenNumbers.push(numbers[i]);
            }
        }
    }
}

contract FilterToMemory {
    function filterEven(uint256[] memory numbers) external pure returns (uint256[] memory) {
        uint256 arrSize = 0;

        for (uint256 i = 0; i < numbers.length; i++) {
            if (numbers[i] % 2 == 0) {
                arrSize++;
            }
        }

        uint256[] memory res = new uint[](arrSize);

        uint256 currIdx = 0;
        for (uint256 i = 0; i < numbers.length; i++) {
            if (numbers[i] % 2 == 0) {
                res[currIdx] = numbers[i];
                currIdx++;
            }
        }

        return res;
    }
}

contract StackClub {
    address[] public members;
    mapping(address => bool) public isMember;

    constructor() {
        members.push(msg.sender);
        isMember[msg.sender] = true;
    }

    function addMember(address _newMember) external {
        require(isMember[msg.sender]);

        isMember[_newMember] = true;
        members.push(_newMember);
    }

    function removeLastMember() external {
        require(isMember[msg.sender]);
        require(members.length > 1);

        address noMember = members[members.length - 1];
        members.pop();
        delete isMember[noMember];
    }
}
