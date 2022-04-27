import { MsgExecuteContract } from '@terra-money/terra.js';
import { client, wallets } from '../library.js';

const contract = "terra13v3ncryrfhpfrk6a2lvsn0uhnz64lwqxwqt2tp";
const wallet = wallets.brianKey;

const execute = new MsgExecuteContract(
    wallet.key.accAddress, // sender
    contract, // contract account address
    { transfer: { recipient: "terra146zqjvvgzmd9slf2pjagvc5jzwktym7ese73w2", amount: "2" } }, // handle msg
    { uluna: 100000 } // coins
);

const executeTx = await wallet.createAndSignTx({
    msgs: [execute]
});

const executeTxResult = await client.tx.broadcast(executeTx);

console.log(executeTxResult);