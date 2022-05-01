import { MsgExecuteContract } from '@terra-money/terra.js';
import { client, wallets } from '../library.js';

const contract = "terra18lmfqfdupmlpfktxc8vple3uf6yg6t00dguxzv";
const wallet = wallets.brianKey;

const execute = new MsgExecuteContract(
    wallet.key.accAddress, // sender
    contract, // contract account address
    { transfer: { recipient: "terra1t63nqq6xevumff4gaw6lu72gj27nf6a3nl9dkz", amount: "2000000000" } }, // handle msg
    { uluna: 10000 } // coins
);

const executeTx = await wallet.createAndSignTx({
    msgs: [execute]
});

const executeTxResult = await client.tx.broadcast(executeTx);

console.log(executeTxResult);