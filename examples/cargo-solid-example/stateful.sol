pragma solidity >=0.5.15 <0.7.0;
// Must use experimental features to support `string[]` in public API
pragma experimental ABIEncoderV2;

contract StatefulContract {
    // the contract's owner, set in the constructor
    address owner;

    // the message we're storing
    string message;

    bytes random_bytes;
    bytes10 random_bytes10;

    constructor(string memory message_, bytes memory random_bytes_, bytes10 random_bytes10_) public {
        // set the owner of the contract for `kill()`
        owner = msg.sender;
        message = message_;
        random_bytes = random_bytes_;
        random_bytes10 = random_bytes10_;
    }

    function setMessage(string memory message_) public {
        // only allow the owner to update the message
        if (msg.sender != owner) return;
        message = message_;
    }

    function printMessage(string[] memory messages_) public {
        // Do nothign
    }

    function getMessage() public view returns (string memory) {
        return message;
    }

    function getDetailsNamed() public view returns (string memory message_, bytes memory random_bytes_, bytes10 random_bytes10_) {
        return (message, random_bytes, random_bytes10);
    }

    function getDetails() public view returns (string memory, bytes memory, bytes10) {
        return (message, random_bytes, random_bytes10);
    }

    // return a string
    function getMessage2() internal view returns (string memory) {
        return message;
    }
    // recover the funds of the contract
    function kill() public { if (msg.sender == owner) selfdestruct(msg.sender); }
}
