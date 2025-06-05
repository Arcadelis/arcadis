import { SorobanRpc, xdr } from 'soroban-client';
import { Address } from '@stellar/stellar-sdk';

export interface VotingContract {
  createProposal: (title: string, options: string[]) => Promise<void>;
  vote: (proposalId: number, selectedOption: number) => Promise<void>;
  getVotes: (proposalId: number) => Promise<number[]>;
  getResults: (proposalId: number) => Promise<{ option: string; votes: number }[]>;
}

export const initVotingContract = (contractId: string): VotingContract => {
  if (!contractId) {
    throw new Error('Contract ID is required');
  }

  const server = new SorobanRpc.Server(process.env.NEXT_PUBLIC_SOROBAN_RPC_URL || 'https://soroban-testnet.stellar.org');

  const invokeContract = async (method: string, ...params: any[]) => {
    const result = await server.simulateTransaction(() => {
      const contract = new SorobanRpc.Contract(contractId);
      return contract.call(method, ...params);
    });
    return result;
  };

  return {
    createProposal: async (title: string, options: string[]) => {
      await invokeContract('create_proposal', title, options);
    },

    vote: async (proposalId: number, selectedOption: number) => {
      await invokeContract('vote', proposalId, selectedOption);
    },

    getVotes: async (proposalId: number) => {
      const result = await invokeContract('get_votes', proposalId);
      return result as number[];
    },

    getResults: async (proposalId: number) => {
      const result = await invokeContract('get_results', proposalId);
      return result as { option: string; votes: number }[];
    },
  };
};