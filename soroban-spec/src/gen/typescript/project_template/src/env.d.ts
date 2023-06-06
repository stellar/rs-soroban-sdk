interface ImportMetaEnv {
  readonly PUBLIC_SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID: string;
  readonly SOROBAN_INSERT_SCREAMING_SNAKE_CASE_CONTRACT_NAME_HERE_CONTRACT_ID: string;

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