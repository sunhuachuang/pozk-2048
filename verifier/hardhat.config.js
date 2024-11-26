require('@openzeppelin/hardhat-upgrades');
require("@nomicfoundation/hardhat-toolbox");
require("hardhat-contract-sizer");
require("hardhat-gas-reporter");
require("dotenv").config();

const SECRET_KEY = process.env.SECRET_KEY ?? '';

// SAFE: Hardhat default sk
const LOCAL_SK1 = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const LOCAL_SK2 = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
const LOCAL_SK3 = "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
const LOCAL_SK4 = "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  docgen: {
    exclude: ["utils", "interface"],
    pages: "files",
  },
  networks: {
    hardhat: {
      mining: {
        auto: true,
        interval: 2000
      },
    },
    localhost: {
      url: 'http://127.0.0.1:8545',
      accounts: [LOCAL_SK1, LOCAL_SK2, LOCAL_SK3, LOCAL_SK4]
    },
    testnet: {
      url: 'https://opbnb-testnet-rpc.bnbchain.org',
      accounts: [SECRET_KEY]
    },
    mainnet: {
      url: 'https://rpc.zypher.network',
      accounts: [SECRET_KEY]
    },
    l2testnet: {
      url: 'https://opbnb-testnet-rpc.bnbchain.org',
      accounts: [SECRET_KEY]
    },
    l2: {
      url: 'https://rpc.linea.build',
      accounts: [SECRET_KEY]
    },
  },
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: './build/cache',
    artifacts: './build/artifacts',
  },
  contractSizer: {
    alphaSort: true,
    runOnCompile: true,
    disambiguatePaths: false,
  },
  mocha: {
    timeout: 200000
  },
  gasReporter: {
    currency: 'USD',
    enabled: true,
  },
  etherscan: {
    // Your API key for Etherscan
    // Obtain one at https://etherscan.io/
    apiKey: '<api-key>',
  }
};
