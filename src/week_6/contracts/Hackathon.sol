// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract Hackathon {
    struct Project {
        string title;
        uint256[] ratings;
    }

    Project[] projects;

    function findWinner() external view returns (Project memory) {
        Project memory winner;
        uint256 max = 0;

        for (uint256 i = 0; i < projects.length; i++) {
            Project memory p = projects[i];
            uint256 avg = 0;

            for (uint256 r = 0; r < p.ratings.length; r++) {
                avg += p.ratings[r];
            }

            if (avg / p.ratings.length > max) {
                winner = p;
                max = avg / p.ratings.length;
            }
        }

        return winner;
    }

    function newProject(string calldata _title) external {
        projects.push(Project(_title, new uint[](0)));
    }

    function rate(uint256 _idx, uint256 _rating) external {
        projects[_idx].ratings.push(_rating);
    }
}
