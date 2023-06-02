interface ImportMetaEnv {
  readonly PUBLIC_SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID: string;
  readonly SOROBAN_CONTRACT_DATA_EXAMPLE_CONTRACT_ID: string;

  readonly PUBLIC_SOROBAN_NETWORK_NAME: string;
  readonly SOROBAN_NETWORK_NAME: string;

  readonly PUBLIC_SOROBAN_NETWORK_PASSPHRASE: string;
  readonly SOROBAN_NETWORK_PASSPHRASE: string;

  readonly PUBLIC_SOROBAN_RPC_URL: string;
  readonly SOROBAN_RPC_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

declare const Errors: { message: string }[];