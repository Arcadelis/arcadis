import { useState, useCallback } from 'react';
import { useWallet } from '@/context/WalletProvider';
import { initVotingContract, VotingContract } from '@/app/soroban/votingContract';

interface UseVotingState {
  loading: boolean;
  error: string | null;
  success: boolean;
}

interface UseVotingReturn extends UseVotingState {
  createProposal: (title: string, options: string[]) => Promise<void>;
  castVote: (proposalId: number, option: number) => Promise<void>;
  getVotes: (proposalId: number) => Promise<number[]>;
  getResults: (proposalId: number) => Promise<{ option: string; votes: number }[]>;
}

export const useVoting = (): UseVotingReturn => {
  const { isWalletConnected } = useWallet();
  const [state, setState] = useState<UseVotingState>({
    loading: false,
    error: null,
    success: false,
  });

  const contractId = process.env.NEXT_PUBLIC_CONTRACT_ID_VOTING;
  let votingContract: VotingContract | null = null;

  try {
    if (contractId) {
      votingContract = initVotingContract(contractId);
    }
  } catch (error) {
    console.error('Error initializing voting contract:', error);
  }

  const handleTransaction = async <T>(operation: () => Promise<T>): Promise<T | void> => {
    if (!isWalletConnected) {
      setState(prev => ({ ...prev, error: 'Please connect your wallet first' }));
      return;
    }

    if (!votingContract) {
      setState(prev => ({ ...prev, error: 'Contract not initialized' }));
      return;
    }

    setState(prev => ({ ...prev, loading: true, error: null, success: false }));

    try {
      const result = await operation();
      setState(prev => ({ ...prev, loading: false, success: true }));
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Transaction failed';
      setState(prev => ({ ...prev, loading: false, error: errorMessage }));
      throw error;
    }
  };

  const createProposal = useCallback(
    async (title: string, options: string[]) => {
      await handleTransaction(() => votingContract!.createProposal(title, options));
    },
    [votingContract]
  );

  const castVote = useCallback(
    async (proposalId: number, option: number) => {
      await handleTransaction(() => votingContract!.vote(proposalId, option));
    },
    [votingContract]
  );

  const getVotes = useCallback(
    async (proposalId: number) => {
      return handleTransaction(() => votingContract!.getVotes(proposalId)) as Promise<number[]>;
    },
    [votingContract]
  );

  const getResults = useCallback(
    async (proposalId: number) => {
      return handleTransaction(() =>
        votingContract!.getResults(proposalId)
      ) as Promise<{ option: string; votes: number }[]>;
    },
    [votingContract]
  );

  return {
    ...state,
    createProposal,
    castVote,
    getVotes,
    getResults,
  };
};