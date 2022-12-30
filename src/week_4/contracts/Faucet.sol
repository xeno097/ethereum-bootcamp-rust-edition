//SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.0;

contract Faucet {
    function withdraw(uint256 _amount) public {
        require(_amount <= 0.1 ether);
        payable(msg.sender).transfer(_amount);
    }

    receive() external payable {}
}
