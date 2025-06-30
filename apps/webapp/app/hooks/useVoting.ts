"use client"

import { useCallback } from "react"
import { useSorobanContract } from "./useSorobanContract"
import { createVotingContract, type Game } from "@/lib/soroban/votingContract"

export interface UseVotingReturn {
  // State
  state: ReturnType<typeof useSorobanContract>["state"]

  // Contract actions
  createProposal: (title: string) => Promise<boolean>
  castVote: (proposalId: number) => Promise<boolean>
  getVotes: (proposalId: number) => Promise<number>
  getResults: (proposalId: number) => Promise<Game | null>
  getAllProposals: () => Promise<Game[]>
  checkHasVoted: (proposalId: number) => Promise<boolean>

  // Utility
  isConnected: boolean
  walletAddress: string | null
}

export function useVoting(): UseVotingReturn {
  const { state, contract, isConnected, walletAddress, executeContract, readContract } =
    useSorobanContract(createVotingContract)

  /**
   * Create a new proposal
   */
  const createProposal = useCallback(
    async (title: string): Promise<boolean> => {
      if (!title.trim()) {
        return false
      }

      const result = await executeContract(() => contract!.createProposal(walletAddress!, title.trim()), {
        successMessage: "Proposal created successfully!",
        errorMessage: "Failed to create proposal",
      })

      return result !== null
    },
    [contract, walletAddress, executeContract],
  )

  /**
   * Cast a vote for a proposal
   */
  const castVote = useCallback(
    async (proposalId: number): Promise<boolean> => {
      // Check if user has already voted
      const hasVoted = await readContract(() => contract!.hasVoted(walletAddress!, proposalId), false)

      if (hasVoted) {
        return false
      }

      const result = await executeContract(() => contract!.vote(walletAddress!, proposalId), {
        successMessage: "Vote cast successfully!",
        errorMessage: "Failed to cast vote",
      })

      return result !== null
    },
    [contract, walletAddress, executeContract, readContract],
  )

  /**
   * Get votes for a proposal
   */
  const getVotes = useCallback(
    async (proposalId: number): Promise<number> => {
      return await readContract(() => contract!.getVotes(proposalId), 0)
    },
    [contract, readContract],
  )

  /**
   * Get results for a proposal
   */
  const getResults = useCallback(
    async (proposalId: number): Promise<Game | null> => {
      return await readContract(() => contract!.getResults(proposalId), null)
    },
    [contract, readContract],
  )

  /**
   * Check if user has voted for a proposal
   */
  const checkHasVoted = useCallback(
    async (proposalId: number): Promise<boolean> => {
      if (!walletAddress) return false

      return await readContract(() => contract!.hasVoted(walletAddress, proposalId), false)
    },
    [walletAddress, contract, readContract],
  )

  /**
   * Get all proposals
   */
  const getAllProposals = useCallback(async (): Promise<Game[]> => {
    if (!contract) return []

    try {
      const totalProposals = await readContract(() => contract.getTotalProposals(), 0)

      if (totalProposals === 0) return []

      const proposals: Game[] = []
      const proposalPromises = Array.from({ length: totalProposals }, (_, i) =>
        readContract(() => contract.getResults(i + 1), null),
      )

      const results = await Promise.allSettled(proposalPromises)

      results.forEach((result, index) => {
        if (result.status === "fulfilled" && result.value) {
          proposals.push(result.value)
        } else {
          console.warn(`Failed to fetch proposal ${index + 1}`)
        }
      })

      return proposals.sort((a, b) => b.votes - a.votes)
    } catch (error) {
      console.error("Error getting all proposals:", error)
      return []
    }
  }, [contract, readContract])

  return {
    state,
    createProposal,
    castVote,
    getVotes,
    getResults,
    getAllProposals,
    checkHasVoted,
    isConnected,
    walletAddress,
  }
}

export type { Game }
