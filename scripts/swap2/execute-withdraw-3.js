import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    MnemonicKey,
    Coins,
} from "@terra-money/terra.js";

const contract = "terra1t63nqq6xevumff4gaw6lu72gj27nf6a3nl9dkz";
const wallet = wallets.brianKey;

const msg = new MsgExecuteContract(
    wallet.key.accAddress,
    contract, {
        withdraw_step3_send_luna: { amount: 1 },
    },
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);