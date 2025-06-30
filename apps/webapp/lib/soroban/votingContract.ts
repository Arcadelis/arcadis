import { Client } from "./src"

export interface Game {
  id: number
  name: string
  votes: number
  creator: string
}

export interface ContractConfig {
  contractId: string
  rpcUrl: string
  networkPassphrase: string
}

export class VotingContract {
  public client: Client

  constructor(config: ContractConfig) {
    this.client = new Client({
      contractId: config.contractId,
      networkPassphrase: config.networkPassphrase,
      rpcUrl: config.rpcUrl,
    })
  }

  /**
   * Create a proposal (add_game)
   */
  async createProposal(creator: string, title: string) {
    return await this.client.add_game({
      creator,
      name: title,
    })
  }

  /**
   * Cast a vote for a proposal
   */
  async vote(voter: string, proposalId: number) {
    return await this.client.vote({
      voter,
      game_id: proposalId,
    })
  }

  /**
   * Get votes for a specific proposal
   */
  async getVotes(proposalId: number): Promise<number> {
    const tx = await this.client.get_game_votes({
      game_id: proposalId,
    })

    const result = await tx.simulate()
    return result.result || 0
  }

  /**
   * Get results for a specific proposal
   */
  async getResults(proposalId: number): Promise<Game | null> {
    try {
      const tx = await this.client.get_game_info({
        game_id: proposalId,
      })

      const result = await tx.simulate()

      if (result.result) {
        const [id, name, votes, creator] = result.result
        return {
          id: Number(id),
          name,
          votes: Number(votes),
          creator,
        }
      }
      return null
    } catch (error) {
      console.error("Error getting results:", error)
      return null
    }
  }

  /**
   * Check if user has voted for a proposal
   */
  async hasVoted(user: string, proposalId: number): Promise<boolean> {
    try {
      const tx = await this.client.has_voted({
        user,
        game_id: proposalId,
      })

      const result = await tx.simulate()
      return result.result || false
    } catch (error) {
      console.error("Error checking vote status:", error)
      return false
    }
  }

  /**
   * Get total number of proposals
   */
  async getTotalProposals(): Promise<number> {
    try {
      const tx = await this.client.get_total_games()
      const result = await tx.simulate()
      return result.result || 0
    } catch (error) {
      console.error("Error getting total proposals:", error)
      return 0
    }
  }
}

/**
 * Factory function to create contract instance
 */
export function createVotingContract(): VotingContract {
  const contractId = process.env.NEXT_PUBLIC_CONTRACT_ID_VOTING
  const network = process.env.NEXT_PUBLIC_STELLAR_NETWORK || "testnet"
  const rpcUrl = process.env.NEXT_PUBLIC_SOROBAN_RPC_URL || "https://soroban-testnet.stellar.org"

  if (!contractId) {
    throw new Error("NEXT_PUBLIC_CONTRACT_ID_VOTING is not configured")
  }

  const networkPassphrase =
    network === "mainnet" ? "Public Global Stellar Network ; September 2015" : "Test SDF Network ; September 2015"

  return new VotingContract({
    contractId,
    rpcUrl,
    networkPassphrase,
  })
}
