import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    Coins,
} from "@terra-money/terra.js";

const contract = "terra18aj0e4hdspspzz9hp0skdtvefzhf9xpfpvreh3";
const wallet = wallets.brianKey;


const amount = (0.5 * 1e6).toFixed(0);

const msg = new MsgExecuteContract(
    wallet.key.accAddress,
    contract, {
        withdraw: {},
    },
    new Coins({ uluna: 1000 }),
);

const tx = await wallet.createAndSignTx({ msgs: [msg] });
const result = await client.tx.broadcast(tx);

console.log(result);