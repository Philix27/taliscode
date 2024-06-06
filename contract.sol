// Quest testnet address
// 0xB8cd7Eb30184C896Fe605d2fd8B64Ce6a4f03DB1
// Mainnet contract
// 0xf5d922385dEf8cc1f05416e88fF03e8C9B7ADB2E
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract InventoryManager {
    struct Item {
        uint id;
        string name;
        uint quantity;
        uint price;
    }

    mapping(uint => Item) public items;
    uint public nextItemId;

    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can perform this action");
        _;
    }

    event ItemAdded(uint id, string name, uint quantity, uint price);
    event ItemUpdated(uint id, string name, uint quantity, uint price);
    event ItemRemoved(uint id);

    constructor() {
        owner = msg.sender;
    }

    function addItem(string memory name, uint quantity, uint price) public onlyOwner {
        items[nextItemId] = Item(nextItemId, name, quantity, price);
        emit ItemAdded(nextItemId, name, quantity, price);
        nextItemId++;
    }

    function updateItem(uint id, string memory name, uint quantity, uint price) public onlyOwner {
        require(id < nextItemId, "Item does not exist");
        items[id] = Item(id, name, quantity, price);
        emit ItemUpdated(id, name, quantity, price);
    }

    function removeItem(uint id) public onlyOwner {
        require(id < nextItemId, "Item does not exist");
        delete items[id];
        emit ItemRemoved(id);
    }

    function getItem(uint id) public view returns (uint, string memory, uint, uint) {
        require(id < nextItemId, "Item does not exist");
        Item memory item = items[id];
        return (item.id, item.name, item.quantity, item.price);
    }
}
