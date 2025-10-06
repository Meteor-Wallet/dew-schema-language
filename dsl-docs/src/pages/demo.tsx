import React from 'react';
import Layout from '@theme/Layout';
import { ethers } from 'ethers';
import z from 'zod';
import Editor from '@monaco-editor/react';
import DewSchemaLanguageWasmInit, {
    DewSchemaLanguageWasmWrapper,
} from 'dsl-web-wasm';

const signatureResultSchema = z.object({
    count: z.number(),
    next: z.string().nullable(),
    previous: z.string().nullable(),
    results: z
        .object({
            id: z.number(),
            created_at: z.string(),
            text_signature: z.string(),
            hex_signature: z.string(),
            bytes_signature: z.string(),
        })
        .array(),
});

function guessSignature(
    txData: string,
    signatures: { id: number; sig: string }[]
) {
    if (!txData.startsWith('0x')) throw new Error('txData must be 0x-prefixed');

    const candidates: Array<{
        id: number;
        sig: string;
        decoded: any;
        exactMatch: boolean;
    }> = [];

    for (const { id, sig } of signatures) {
        try {
            const iface = new ethers.Interface([`function ${sig}`]);
            const decoded = iface.decodeFunctionData(sig, txData);

            // Re-encode arguments to see if they match perfectly
            const reencoded = iface.encodeFunctionData(sig, decoded);

            const exactMatch = reencoded.toLowerCase() === txData.toLowerCase();

            candidates.push({ id, sig, decoded, exactMatch });
        } catch (err) {
            // Ignore invalid
        }
    }

    return candidates
        .filter((c) => c.exactMatch)
        .sort((a, b) => a.sig.length - b.sig.length);
}

function formatDecoded(decoded: any): Record<string, any> {
    const out: Record<string, any> = {};
    decoded.forEach((val: any, i: number) => {
        out[`args${i}`] = normalizeArg(val);
    });
    return out;
}

function normalizeArg(value: any): any {
    if (value == null) return null;

    if (typeof value === 'boolean') return value;

    if (typeof value === 'string') return value; // already address, bytes, etc.

    if (typeof value === 'number') return value; // JS number

    // ethers v6 BigInt
    if (typeof value === 'bigint') {
        return value <= Number.MAX_SAFE_INTEGER
            ? Number(value)
            : value.toString();
    }

    // ethers v5 BigNumber (just in case)
    if (value._isBigNumber) {
        const asBigInt = BigInt(value.toString());
        return asBigInt <= Number.MAX_SAFE_INTEGER
            ? Number(asBigInt)
            : asBigInt.toString();
    }

    // Arrays → normalize each element
    if (Array.isArray(value)) {
        return value.map((v) => normalizeArg(v));
    }

    // Objects (structs) → recurse
    if (typeof value === 'object') {
        const out: Record<string, any> = {};
        for (const [k, v] of Object.entries(value)) {
            // skip ethers Result index keys (numeric duplicates)
            if (!isNaN(Number(k))) continue;
            out[k] = normalizeArg(v);
        }
        return out;
    }

    return String(value);
}

export default function Demo() {
    const [txHash, setTxHash] = React.useState<string>('');
    const [txParseError, setTxParseError] = React.useState<string | null>(null);
    const [rootObject, setRootObject] = React.useState<string>('');
    const isDark =
        document.documentElement.getAttribute('data-theme') === 'dark';
    const [functionSignature, setFunctionSignature] =
        React.useState<string>('');
    const [etherscanApiKey, setEtherscanApiKey] = React.useState<string>('');
    const [schemaRule, setSchemaRule] = React.useState<string>('');
    const [schemaResult, setSchemaResult] = React.useState<string>('');
    const [hostFunctions, setHostFunctions] = React.useState<string>('{\n}');

    React.useEffect(() => {
        const fetchTransaction = async (originalTxHash: string) => {
            if (
                originalTxHash.length !== 66 ||
                !originalTxHash.startsWith('0x')
            ) {
                return;
            }

            const provider = new ethers.JsonRpcProvider(
                'https://eth.rpc.blxrbdn.com'
            );
            const tx = await provider.getTransaction(originalTxHash);

            // user already changed the tx hash, ignore this result
            if (txHash !== originalTxHash) return;

            const functionSelector = tx.data.slice(0, 10);
            console.log('Function Selector:', functionSelector);

            const signatureResult = await fetch(
                'https://www.4byte.directory/api/v1/signatures/?hex_signature=' +
                    functionSelector
            ).then((res) => res.json());

            const signatures = signatureResultSchema
                .parse(signatureResult)
                .results.map((sig) => ({
                    id: sig.id,
                    sig: sig.text_signature,
                }));

            const guessedSignatures = guessSignature(tx.data, signatures);

            const rootObject = {
                to: tx.to,
                value: tx.value.toString(),
                method_name:
                    guessedSignatures[0]?.sig.split('(')[0] || undefined,
                args: guessedSignatures[0]
                    ? formatDecoded(guessedSignatures[0].decoded)
                    : undefined,
            };

            setRootObject(JSON.stringify(rootObject, null, 4));
        };

        setTxParseError(null);
        fetchTransaction(txHash).catch((err) => {
            console.error(err);
            setTxParseError((err as Error).message);
        });
    }, [txHash]);

    React.useEffect(() => {
        try {
            const exec = new Function(`return ${hostFunctions};`);

            const hostFuncs = exec() as Record<string, (...args: any[]) => any>;

            const dsl = new DewSchemaLanguageWasmWrapper(rootObject, hostFuncs);

            const result = dsl.evaluate(schemaRule);

            console.log('DSL Result:', result);

            setSchemaResult(JSON.stringify(result, null, 4));
        } catch (err) {
            setSchemaResult('Error: ' + (err as Error).message);
        }
    }, [rootObject, schemaRule, hostFunctions]);

    return (
        <Layout title='Demo'>
            <div
                style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr 1fr',
                    gap: '2rem',
                    padding: '2rem',
                }}
            >
                {/* LEFT SIDE */}
                <div
                    style={{
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '1rem',
                    }}
                >
                    <label>1st step: Transaction Hash: (Ethereum)</label>
                    <input
                        type='text'
                        value={txHash}
                        onChange={(e) => setTxHash(e.target.value)}
                        placeholder='Enter transaction hash'
                        style={{
                            padding: '0.5rem',
                            border: '1px solid #ccc',
                            borderRadius: '6px',
                        }}
                    />

                    {txParseError && (
                        <p style={{ color: 'red' }}>Error: {txParseError}</p>
                    )}

                    <label>
                        (Optional) Etherscan API Key - for guessing the Function
                        Signature
                    </label>
                    <input
                        type='text'
                        placeholder='Your Etherscan API Key'
                        value={etherscanApiKey}
                        onChange={(e) => setEtherscanApiKey(e.target.value)}
                        style={{
                            padding: '0.5rem',
                            border: '1px solid #ccc',
                            borderRadius: '6px',
                        }}
                    />

                    <label>(Optional) Function Signature</label>
                    <input
                        type='text'
                        placeholder='transfer(address receiver, uint256 amount)'
                        value={functionSignature}
                        onChange={(e) => setFunctionSignature(e.target.value)}
                        style={{
                            padding: '0.5rem',
                            border: '1px solid #ccc',
                            borderRadius: '6px',
                        }}
                    />

                    <label>2nd step: Edit the Root Object:</label>
                    <Editor
                        height='250px'
                        defaultLanguage='json'
                        value={rootObject}
                        onChange={(value) => setRootObject(value || '')}
                        theme={isDark ? 'vs-dark' : 'light'}
                        options={{ minimap: { enabled: false } }}
                    />

                    <label>3rd step: (Optional) Host Functions:</label>
                    <Editor
                        height='150px'
                        defaultLanguage='javascript'
                        value={hostFunctions}
                        onChange={(value) => setHostFunctions(value || '')}
                        theme={isDark ? 'vs-dark' : 'light'}
                        options={{ minimap: { enabled: false } }}
                    />
                </div>

                {/* RIGHT SIDE */}
                <div
                    style={{
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '1rem',
                    }}
                >
                    <label>4th step: Edit the Schema Rule:</label>
                    <Editor
                        height='400px'
                        defaultLanguage='javascript'
                        value={schemaRule}
                        onChange={(value) => setSchemaRule(value || '')}
                        theme={isDark ? 'vs-dark' : 'light'}
                        options={{ minimap: { enabled: false } }}
                        beforeMount={(monaco) => {
                            // 1. Remove the default JS libs
                            monaco.languages.typescript.javascriptDefaults.setCompilerOptions(
                                {
                                    noLib: true, // disable built-in `lib.d.ts`
                                    allowNonTsExtensions: true,
                                }
                            );

                            // 2. Add your own DSL typings
                            fetch('/dsl.d.ts')
                                .then((res) => res.text())
                                .then((dts) => {
                                    monaco.languages.typescript.javascriptDefaults.addExtraLib(
                                        dts,
                                        'file:///node_modules/@types/dsl/index.d.ts'
                                    );
                                });
                        }}
                    />

                    <label>
                        5th step: Result: (any result other than true usually
                        means rejected)
                    </label>
                    <Editor
                        height='200px'
                        defaultLanguage='json'
                        value={schemaResult}
                        theme={isDark ? 'vs-dark' : 'light'}
                        options={{
                            minimap: { enabled: false },
                            readOnly: true,
                        }}
                    />
                </div>
            </div>
        </Layout>
    );
}
