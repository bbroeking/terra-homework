import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    MnemonicKey,
    Coins,
} from "@terra-money/terra.js";

const contract = "terra1t63nqq6xevumff4gaw6lu72gj27nf6a3nl9dkz";
const wallet = wallets.brianKey;

const amount = (0.1 * 1e6).toFixed(0);

const msg = new MsgExecuteContract(
    wallet.key.accAddress,
    contract, {
        start_undelegation: { amount: amount },
    },
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);