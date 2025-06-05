'use client';

import { useState, useEffect } from 'react';
import { useWallet } from '@/context/WalletProvider';
import { useVoting } from '@/app/hooks/useVoting';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

interface Proposal {
  id: number;
  title: string;
  options: string[];
  results: { option: string; votes: number }[];
}

export default function GovernancePage() {
  const { isWalletConnected, connectWallet } = useWallet();
  const { createProposal, castVote, getResults, loading, error, success } = useVoting();

  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [newProposal, setNewProposal] = useState({
    title: '',
    options: ['', ''] // Minimum two options
  });

  // In a real application, you would fetch existing proposals here
  useEffect(() => {
    // Fetch proposals logic would go here
  }, []);

  const handleCreateProposal = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createProposal(newProposal.title, newProposal.options.filter(opt => opt.trim()));
      setNewProposal({ title: '', options: ['', ''] });
      // Refresh proposals list
    } catch (error) {
      console.error('Error creating proposal:', error);
    }
  };

  const handleVote = async (proposalId: number, optionIndex: number) => {
    try {
      await castVote(proposalId, optionIndex);
      // Refresh proposal results
      const results = await getResults(proposalId);
      setProposals(prev =>
        prev.map(p =>
          p.id === proposalId ? { ...p, results } : p
        )
      );
    } catch (error) {
      console.error('Error voting:', error);
    }
  };

  const addOption = () => {
    setNewProposal(prev => ({
      ...prev,
      options: [...prev.options, '']
    }));
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold mb-8">Governance</h1>

      {!isWalletConnected ? (
        <div className="text-center py-8">
          <p className="mb-4">Connect your wallet to participate in governance</p>
          <Button onClick={connectWallet}>Connect Wallet</Button>
        </div>
      ) : (
        <>
          {/* Create Proposal Form */}
          <div className="bg-gray-100 p-6 rounded-lg mb-8">
            <h2 className="text-2xl font-semibold mb-4">Create New Proposal</h2>
            <form onSubmit={handleCreateProposal}>
              <div className="mb-4">
                <label className="block mb-2">Title</label>
                <Input
                  type="text"
                  value={newProposal.title}
                  onChange={(e) => setNewProposal(prev => ({ ...prev, title: e.target.value }))}
                  required
                />
              </div>
              
              <div className="mb-4">
                <label className="block mb-2">Options</label>
                {newProposal.options.map((option, index) => (
                  <Input
                    key={index}
                    type="text"
                    className="mb-2"
                    value={option}
                    onChange={(e) => {
                      const newOptions = [...newProposal.options];
                      newOptions[index] = e.target.value;
                      setNewProposal(prev => ({ ...prev, options: newOptions }));
                    }}
                    required
                  />
                ))}
                <Button
                  type="button"
                  variant="outline"
                  onClick={addOption}
                  className="mt-2"
                >
                  Add Option
                </Button>
              </div>

              <Button type="submit" disabled={loading}>
                {loading ? 'Creating...' : 'Create Proposal'}
              </Button>

              {error && <p className="text-red-500 mt-2">{error}</p>}
              {success && <p className="text-green-500 mt-2">Proposal created successfully!</p>}
            </form>
          </div>

          {/* Proposals List */}
          <div className="space-y-6">
            {proposals.map((proposal) => (
              <div key={proposal.id} className="bg-white p-6 rounded-lg shadow">
                <h3 className="text-xl font-semibold mb-4">{proposal.title}</h3>
                <div className="space-y-4">
                  {proposal.options.map((option, index) => {
                    const result = proposal.results?.find(r => r.option === option);
                    const totalVotes = proposal.results?.reduce((sum, r) => sum + r.votes, 0) || 0;
                    const percentage = totalVotes > 0 ? (result?.votes || 0) / totalVotes * 100 : 0;

                    return (
                      <div key={index} className="space-y-2">
                        <div className="flex justify-between items-center">
                          <span>{option}</span>
                          <Button
                            onClick={() => handleVote(proposal.id, index)}
                            disabled={loading}
                            variant="outline"
                          >
                            Vote
                          </Button>
                        </div>
                        <div className="w-full bg-gray-200 rounded-full h-2.5">
                          <div
                            className="bg-blue-600 h-2.5 rounded-full"
                            style={{ width: `${percentage}%` }}
                          />
                        </div>
                        <div className="text-sm text-gray-500">
                          {result?.votes || 0} votes ({percentage.toFixed(1)}%)
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}