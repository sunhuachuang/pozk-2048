// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { attachContract, sleep } = require("./address_utils.js");
const { writeFile } = require('fs');

const NAME = "Game2048Step60CircomVerifier";
const ADDR = "0x5b92b011513f9aaf8f6541003dc088625e7438e8";

async function upgrade() {
  const C = await ethers.getContractFactory(NAME);
  const address = await C.attach(ADDR);
  const Factory = await ethers.getContractFactory(NAME);
  console.log(`${NAME} upgrading...`);
  await upgrades.upgradeProxy(address, Factory);
  console.log(`${NAME} upgraded`);
}

async function main() {
  await upgrade();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
