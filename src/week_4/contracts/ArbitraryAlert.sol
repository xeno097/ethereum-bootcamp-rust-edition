// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Sidekick {
    function relay(address hero, bytes memory data) external {
        (bool ok,) = hero.call(data);
        require(ok);
    }
}

contract Hero {
    Ambush public ambush;

    struct Ambush {
        bool alerted;
        uint256 enemies;
        bool armed;
    }

    uint256 lastContact;

    function alert(uint256 enemies, bool armed) external {
        ambush = Ambush(true, enemies, armed);
    }
}
