// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IHero {
    function alert() external;
}

contract Sidekick {
    function sendAlert(address hero) external {
        IHero(hero).alert();
    }
}

// In the orginal content this contract was stored in another file
contract Hero {
    bool public alerted;

    function alert() external {
        alerted = true;
    }
}
