import { client, wallets } from '../library.js';

const contract = "terra1x8djvfkqgz5nmq63vd08vr2apteahffg48dy8h";

const response = await client.wasm.contractQuery(contract, { query_token_address: {} });

console.log(response);