// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Setter {
	uint public value;

	function modify(uint _value) external {
		value = _value;
	}
}