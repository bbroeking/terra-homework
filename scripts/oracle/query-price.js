import { client, wallets } from '../library.js';

import {
    MsgExecuteContract,
    MnemonicKey,
    Coins,
    LCDClient,
} from "@terra-money/terra.js";

const cw20Contract = "terra1sun0cavpuzqrwgg4kqpvs6d0yfj60f34u3agvz";

const response = await client.wasm.contractQuery(cw20Contract, { query_price: {} });

console.log(response);