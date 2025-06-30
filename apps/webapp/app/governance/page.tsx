"use client"

import type React from "react"
import { useState, useEffect } from "react"
import { useVoting, type Game } from "../hooks/useVoting"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Badge } from "@/components/ui/badge"
import { Separator } from "@/components/ui/separator"
import { Progress } from "@/components/ui/progress"
import { Plus, Vote, Trophy, Users, Zap, BarChart3, RefreshCw } from "lucide-react"
import { Wallet } from "lucide-react" // Declare the Wallet variable

export default function GovernancePage() {
  const [proposals, setProposals] = useState<Game[]>([])
  const [newProposalTitle, setNewProposalTitle] = useState("")
  const [votedProposals, setVotedProposals] = useState<Set<number>>(new Set())
  const [refreshing, setRefreshing] = useState(false)

  const { state, createProposal, castVote, getAllProposals, checkHasVoted, isConnected, walletAddress } = useVoting()

  // Load proposals and voting status
  const loadProposals = async () => {
    setRefreshing(true)
    try {
      const allProposals = await getAllProposals()
      setProposals(allProposals)

      // Check voting status for each proposal
      if (isConnected && walletAddress) {
        const votedSet = new Set<number>()
        for (const proposal of allProposals) {
          const hasVoted = await checkHasVoted(proposal.id)
          if (hasVoted) {
            votedSet.add(proposal.id)
          }
        }
        setVotedProposals(votedSet)
      }
    } catch (err) {
      console.error("Error loading proposals:", err)
    } finally {
      setRefreshing(false)
    }
  }

  useEffect(() => {
    loadProposals()
  }, [isConnected, walletAddress])

  const handleCreateProposal = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!newProposalTitle.trim()) return

    const success = await createProposal(newProposalTitle.trim())
    if (success) {
      setNewProposalTitle("")
      await loadProposals() // Refresh the list
    }
  }

  const handleVote = async (proposalId: number) => {
    const success = await castVote(proposalId)
    if (success) {
      await loadProposals() // Refresh the list
    }
  }

  const truncateAddress = (address: string) => `${address.slice(0, 6)}...${address.slice(-4)}`

  // Calculate total votes for percentage calculation
  const totalVotes = proposals.reduce((sum, proposal) => sum + proposal.votes, 0)

  if (!isConnected) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h1 className="text-3xl font-bold mb-4">Governance & Voting</h1>
          <p className="text-muted-foreground mb-8">Connect your wallet to participate in governance voting</p>
          <Card className="max-w-md mx-auto">
            <CardContent className="pt-6">
              <div className="text-center">
                <Wallet className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p className="text-muted-foreground mb-4">
                  Please connect your Stellar wallet to create proposals and vote
                </p>
                <p className="text-sm text-muted-foreground">Supports Freighter wallet on Testnet and Futurenet</p>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">Governance & Voting</h1>
        <p className="text-muted-foreground">Create proposals and vote on the future of Arcadelis</p>

        {/* Contract Status */}
        <Card className="mt-4 border-green-200 bg-green-50">
          <CardContent className="pt-4">
            <div className="flex items-center gap-2 text-green-800">
              <Zap className="h-4 w-4" />
              <p className="text-sm">
                <strong>Live Contract:</strong> Connected to Soroban voting contract{" "}
                <code className="bg-green-100 px-1 rounded text-xs">
                  {process.env.NEXT_PUBLIC_CONTRACT_ID_VOTING?.slice(0, 8)}...
                </code>
              </p>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Create Proposal Form */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Plus className="h-5 w-5" />
            Create New Proposal
          </CardTitle>
          <CardDescription>Submit a new proposal for community voting</CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleCreateProposal} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="proposalTitle">Proposal Title</Label>
              <Input
                id="proposalTitle"
                placeholder="Enter proposal title..."
                value={newProposalTitle}
                onChange={(e) => setNewProposalTitle(e.target.value)}
                disabled={state.loading}
              />
            </div>
            <Button type="submit" disabled={state.loading || !newProposalTitle.trim()}>
              {state.loading ? "Creating..." : "Create Proposal"}
            </Button>
          </form>
          {state.error && <p className="text-sm text-destructive mt-2">{state.error}</p>}
        </CardContent>
      </Card>

      {/* Proposals List */}
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <h2 className="text-2xl font-semibold flex items-center gap-2">
            <BarChart3 className="h-6 w-6" />
            Proposals ({proposals.length})
          </h2>
          <Button variant="outline" onClick={loadProposals} disabled={refreshing}>
            <RefreshCw className={`h-4 w-4 mr-2 ${refreshing ? "animate-spin" : ""}`} />
            {refreshing ? "Refreshing..." : "Refresh"}
          </Button>
        </div>

        {proposals.length === 0 ? (
          <Card>
            <CardContent className="pt-6">
              <div className="text-center text-muted-foreground">
                <Trophy className="h-12 w-12 mx-auto mb-4 opacity-50" />
                <p>No proposals created yet. Be the first to submit a proposal!</p>
              </div>
            </CardContent>
          </Card>
        ) : (
          <div className="grid gap-6">
            {proposals.map((proposal, index) => {
              const votePercentage = totalVotes > 0 ? (proposal.votes / totalVotes) * 100 : 0
              const isTopProposal = index === 0 && proposal.votes > 0

              return (
                <Card key={proposal.id} className={`relative ${isTopProposal ? "border-yellow-200 bg-yellow-50" : ""}`}>
                  {isTopProposal && (
                    <div className="absolute -top-2 -right-2">
                      <Badge className="bg-yellow-500 text-yellow-900">
                        <Trophy className="h-3 w-3 mr-1" />
                        Leading
                      </Badge>
                    </div>
                  )}

                  <CardHeader>
                    <div className="flex items-start justify-between">
                      <div className="flex-1">
                        <CardTitle className="text-xl mb-2">{proposal.name}</CardTitle>
                        <CardDescription>
                          Proposed by {truncateAddress(proposal.creator)} • Proposal #{proposal.id}
                        </CardDescription>
                      </div>
                      <Badge variant="secondary" className="flex items-center gap-1">
                        <Users className="h-3 w-3" />
                        {proposal.votes} votes
                      </Badge>
                    </div>
                  </CardHeader>

                  <CardContent>
                    {/* Vote Progress */}
                    <div className="mb-4">
                      <div className="flex justify-between text-sm text-muted-foreground mb-2">
                        <span>Vote Share</span>
                        <span>{votePercentage.toFixed(1)}%</span>
                      </div>
                      <Progress value={votePercentage} className="h-2" />
                    </div>

                    <Separator className="mb-4" />

                    <div className="flex items-center justify-between">
                      <div className="text-sm text-muted-foreground">{proposal.votes} total votes</div>
                      <Button
                        size="sm"
                        onClick={() => handleVote(proposal.id)}
                        disabled={state.loading || votedProposals.has(proposal.id)}
                        className="flex items-center gap-1"
                        variant={votedProposals.has(proposal.id) ? "outline" : "default"}
                      >
                        <Vote className="h-3 w-3" />
                        {votedProposals.has(proposal.id) ? "Voted" : "Vote"}
                      </Button>
                    </div>

                    {votedProposals.has(proposal.id) && (
                      <Badge variant="outline" className="mt-3 w-full justify-center text-green-700 border-green-200">
                        ✓ You voted for this proposal
                      </Badge>
                    )}
                  </CardContent>
                </Card>
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
