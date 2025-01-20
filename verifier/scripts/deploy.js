// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const { ethers, upgrades, network } = require("hardhat");
const { writeFile } = require('fs');

// zytron testnet
// const ADDR = "0x8aCF473885f975023e7A9B7C043Da53D690cbc6E";

// base sepolia
const ADDR = "0x01156BB7c9b003Ce2E2d2b0EE934b2BAF196B08c";

// mainnet

async function deployContractWithProxy(name, params=[]) {
  const Factory = await ethers.getContractFactory(name);
  //  use upgradeable deploy, then contracts can be upgraded success, otherwise will get error about ERC 1967 proxy
  const contract = await upgrades.deployProxy(Factory, params);
  await contract.waitForDeployment();
  const address = await contract.getAddress();
  console.log(`${name} address: ${address}`);

  return address;
}

async function deploy() {
  const _c = await deployContractWithProxy("Game2048Step60CircomVerifier", []);
}

async function upgrade() {
  console.log("upgrading...");
  const C = await ethers.getContractFactory("Game2048Step60CircomVerifier");
  const address = await C.attach(ADDR);
  const Factory = await ethers.getContractFactory("Game2048Step60CircomVerifier");
  await upgrades.upgradeProxy(address, Factory);
  console.log("upgraded");
}

async function main() {
  await deploy();
  // await upgrade();
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
