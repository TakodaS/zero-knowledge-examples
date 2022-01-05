
pragma solidity ^0.8.0;

import "./verifier.sol";

contract Hashexample is Verifier {
  address payable receiver;

  function claim(Proof memory proof) public   {
    bool validTx = this.verifyTx(proof);
      require(validTx, "Proof submitted was not valid!"); //error message not printed in truffle
      address payable sender = payable(msg.sender);
      sender.transfer(address(this).balance);
  }

  receive() external payable { }
}
