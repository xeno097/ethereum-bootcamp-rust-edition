// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Voting {
    struct Proposal {
        address target;
        bytes data;
        uint256 yesCount;
        uint256 noCount;
    }

    struct Vote {
        bool vote;
        bool voted;
    }

    mapping(uint256 => mapping(address => Vote)) voted;
    mapping(address => bool) public canVote;
    uint256 threshold = 10;
    Proposal[] public proposals;

    constructor(address[] memory _voters) {
        canVote[msg.sender] = true;

        for (uint256 i = 0; i < _voters.length; i++) {
            canVote[_voters[i]] = true;
        }
    }

    event ProposalCreated(uint256);
    event VoteCast(uint256, address);

    modifier checkVote() {
        require(canVote[msg.sender]);
        _;
    }

    function newProposal(address _target, bytes calldata _calldata) external checkVote {
        emit ProposalCreated(proposals.length);

        proposals.push(Proposal(_target, _calldata, 0, 0));
    }

    function castVote(uint256 _proposal, bool _vote) external checkVote {
        if (!voted[_proposal][msg.sender].voted) {
            _castVote(_proposal, _vote);
        } else {
            _updateVote(_proposal, _vote);
        }

        emit VoteCast(_proposal, msg.sender);

        if (proposals[_proposal].yesCount == threshold) {
            (bool ok,) = proposals[_proposal].target.call(proposals[_proposal].data);
            require(ok);
        }
    }

    function _castVote(uint256 _proposal, bool _vote) private {
        if (_vote) {
            proposals[_proposal].yesCount++;
        } else {
            proposals[_proposal].noCount++;
        }

        voted[_proposal][msg.sender] = Vote({vote: _vote, voted: true});
    }

    function _updateVote(uint256 _proposal, bool _vote) private {
        if (voted[_proposal][msg.sender].vote) {
            proposals[_proposal].yesCount--;
        } else {
            proposals[_proposal].noCount--;
        }

        if (_vote) {
            proposals[_proposal].yesCount++;
        } else {
            proposals[_proposal].noCount++;
        }

        voted[_proposal][msg.sender].vote = _vote;
    }
}

contract DummyExecutor {
    uint256 public minted = 0;

    function mint(uint256 amount) external {
        minted += amount;
    }
}
