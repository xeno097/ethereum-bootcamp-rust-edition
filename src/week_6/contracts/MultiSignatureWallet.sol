// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract MultiSig {
    address[] public owners;
    uint256 public required;

    struct Transaction {
        address to;
        uint256 amount;
        bool executed;
        bytes data;
    }

    Transaction[] public transactions;
    mapping(uint => mapping(address => bool)) public confirmations;
    mapping(uint => uint) public confirmationsCounter;
    mapping(address => bool) isOwner;

    constructor(address[] memory _owners, uint256 _required) {
        require(_owners.length > 0);
        require(_required > 0);
        require(_owners.length > _required);

        for (uint i =0;i< _owners.length;i++){
            isOwner[_owners[i]]=true;
        }
        
        owners = _owners;
        required = _required;

    }

    function transactionCount() public view returns(uint256){
        return transactions.length;
    }

    function submitTransaction(address _to, uint _value, bytes memory _data) external returns(uint256){
        uint256 id = addTransaction(_to,_value,_data);

        confirmTransaction(id);

        return id;
    }

    function addTransaction(address _to, uint256 _value, bytes memory _data) internal returns(uint256){
        transactions.push(Transaction(_to,_value,false, _data));

        return transactionCount() - 1;
    }

    function confirmTransaction(uint256 _id) public {
        require(!confirmations[_id][msg.sender] && isOwner[msg.sender]);
        confirmations[_id][msg.sender] = true;
        confirmationsCounter[_id]++;

        if(isConfirmed(_id)){
            executeTransaction(_id);
        }
    }

    function executeTransaction(uint transactionId) public {
        require(isOwner[msg.sender]);
        require(isConfirmed(transactionId));

        Transaction memory pendingTx = transactions[transactionId];

        require(!pendingTx.executed);

        pendingTx.executed = true;
        transactions[transactionId] = pendingTx;

        (bool ok,) = pendingTx.to.call{value: pendingTx.amount}(pendingTx.data);
        require(ok);
    }

    function getConfirmationsCount(uint transactionId) public view returns(uint256){
        return confirmationsCounter[transactionId];
    }

    function isConfirmed(uint transactionId) public view returns(bool){
        return getConfirmationsCount(transactionId) == required;
    }

    receive() external payable {}
}


contract ERC20 {
    uint public totalSupply;
    string public name = "SomeToken";
    string public symbol = "SMT";
    uint8 public decimals = 18;

    mapping(address => uint256) public balanceOf;

    event Transfer(address from, address to, uint256 value);

    constructor() {
        totalSupply = 1000 * (10 ** decimals);
        balanceOf[msg.sender] = totalSupply;
    }

    function transfer(address _to, uint _value) external returns(bool){
        require(balanceOf[msg.sender] >= _value);

        balanceOf[msg.sender]-=_value;
        balanceOf[_to]+=_value;

        emit Transfer(msg.sender,_to,_value);

        return true;
    }

}