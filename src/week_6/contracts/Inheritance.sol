// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

interface IEnemy {
	function takeAttack(Hero.AttackTypes attackType) external;
}

contract Enemy {

	event Attack(Hero.AttackTypes attackType);

	function takeAttack(Hero.AttackTypes attackType) external{
		emit Attack(attackType);
	}
}

contract Hero {
	uint public health;
	uint public energy = 10;

	constructor(uint _health) {
		health = _health;
	}

	enum AttackTypes { Brawl, Spell }
	
	function attack(address) public virtual {
		energy--;
	}

		function takeDamage(uint damage) external {
		health -= damage;
	}
}

contract Mage is Hero(50) {
    function attack(address _enemy) public override {
         IEnemy(_enemy).takeAttack(AttackTypes.Spell);
         super.attack(_enemy);
    }
}

contract Warrior is Hero(200) {

    function attack(address _enemy) public override {
        IEnemy(_enemy).takeAttack(AttackTypes.Brawl);
        super.attack(_enemy);
    }
}

