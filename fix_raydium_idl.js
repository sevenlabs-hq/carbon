const fs = require('fs');

// Read the original IDL
const idl = JSON.parse(fs.readFileSync('raydium_amm.json', 'utf8'));

// Extract Fees from accounts section
const feesDefinition = {
    name: 'Fees',
    type: {
        kind: 'struct',
        fields: [
            {
                name: 'minSeparateNumerator',
                type: 'u64',
            },
            {
                name: 'minSeparateDenominator',
                type: 'u64',
            },
            {
                name: 'tradeFeeNumerator',
                type: 'u64',
            },
            {
                name: 'tradeFeeDenominator',
                type: 'u64',
            },
            {
                name: 'pnlNumerator',
                type: 'u64',
            },
            {
                name: 'pnlDenominator',
                type: 'u64',
            },
            {
                name: 'swapFeeNumerator',
                type: 'u64',
            },
            {
                name: 'swapFeeDenominator',
                type: 'u64',
            },
        ],
    },
};

// Remove Fees from accounts section
idl.accounts = idl.accounts.filter(account => account.name !== 'Fees');

// Add Fees to the beginning of types section
idl.types.unshift(feesDefinition);

// Write the modified IDL
fs.writeFileSync('raydium_amm_fixed.json', JSON.stringify(idl, null, 2));

console.log('✅ Moved Fees from accounts to types section');
console.log('📁 Created raydium_amm_fixed.json');
console.log(`📊 Accounts count: ${idl.accounts.length}`);
console.log(`📊 Types count: ${idl.types.length}`);
