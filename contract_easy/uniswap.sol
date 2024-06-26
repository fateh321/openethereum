 pragma solidity >=0.4.22 <0.6.0;


contract uniswap { 

    uint256 totalSupply_;
    mapping(address => uint256) balances;

    constructor(uint256 total) public {  
        totalSupply_ = total;
        balances[msg.sender] = totalSupply_;
    }  

    function dtransfer(address _contract1, address _contract2, address receiver, uint numTokens) external returns (bool) {
  //  balances[msg.sender] = balances[msg.sender]-(numTokens);
    erc20(_contract1).transfer(receiver, numTokens);
    erc20(_contract2).transfer(receiver, numTokens);
    return true;
    }
    
}


contract erc20 {

    string public constant name = "ERC20Basic";
    string public constant symbol = "BSC";
    uint8 public constant decimals = 18;  


    event Approval(address indexed tokenOwner, address indexed spender, uint tokens);
    event Transfer(address indexed from, address indexed to, uint tokens);


    mapping(address => uint256) balances;

    mapping(address => mapping (address => uint256)) allowed;
    
    uint256 totalSupply_;
    address _owner ;


   constructor(uint256 total) public {  
    totalSupply_ = total;
    balances[msg.sender] = totalSupply_;
    _owner = msg.sender;
    }  

    function totalSupply() public view returns (uint256) {
    return totalSupply_;
    }
    
    function balanceOf(address tokenOwner) public view returns (uint) {
        return balances[tokenOwner];
    }

    function transfer(address receiver, uint numTokens) public returns (bool) {
        require(numTokens <= balances[_owner]);
        balances[_owner] = balances[_owner]-(numTokens);
        balances[receiver] = balances[receiver]+(numTokens);
        emit Transfer(_owner, receiver, numTokens);
        return true;
    }

    function approve(address delegate, uint numTokens) public returns (bool) {
        allowed[msg.sender][delegate] = numTokens;
      emit  Approval(msg.sender, delegate, numTokens);
        return true;
    }

    function allowance(address owner, address delegate) public view returns (uint) {
        return allowed[owner][delegate];
    }

    function transferFrom(address owner, address buyer, uint numTokens) public returns (bool) {
        require(numTokens <= balances[owner]);    
        //require(numTokens <= allowed[owner][msg.sender]);
    
        balances[owner] = balances[owner]-(numTokens);
        //allowed[owner][msg.sender] = allowed[owner][msg.sender]-(numTokens);
        balances[buyer] = balances[buyer]+(numTokens);
      emit  Transfer(owner, buyer, numTokens);
        return true;
    }
}

