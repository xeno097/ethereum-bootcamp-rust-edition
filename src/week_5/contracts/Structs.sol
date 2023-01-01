// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract VoteStorage {
	enum Choices { Yes, No }

	struct Vote {
		Choices choice;
		address voter;
	}

	Vote public vote;

	function createVote(Choices choice) external {
		vote.choice = choice;
		vote.voter = msg.sender;
	}
}

contract VoteMemory {
	enum Choices { Yes, No }

	struct Vote {
		Choices choice;
		address voter;
	}
	
	function createVote(Choices choice) external view returns(Vote memory) {
		return Vote(choice, msg.sender);
	}
}


contract VoteArray {
	enum Choices { Yes, No }
	
	struct Vote {
		Choices choice;
		address voter;
	}
	
	Vote[] public votes;

	function createVote(Choices choice) external {
		require(!hasVoted(msg.sender));
		votes.push(Vote(choice,msg.sender));
	}

	function hasVoted(address _voter) public view returns(bool){
		(Vote memory vote, ) = _getVote(_voter);
		
		return vote.voter == _voter;
	}

	function _getVote(address _voter) private view returns(Vote memory vote, uint idx){
		for(uint i=0;i< votes.length;i++){
			if(votes[i].voter == _voter){
				vote = votes[i];
				idx = i;
			}
		}
	}

	function findChoice(address _voter) external view returns(Choices){
		(Vote memory vote, ) = _getVote(_voter);
		
		return vote.choice;
	}

	function changeVote(Choices _choice) external {
		(, uint idx) = _getVote(msg.sender);

		require(votes[idx].voter == msg.sender);

		votes[idx].choice = _choice;
		
	}
}