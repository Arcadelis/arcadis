import TournamentList from "@/components/TournamentLists";

const Community = () => {
  // Sample tournament data
  const tournaments = [
    {
      id: "1",
      name: "Winter Championship",
      prizePool: 20000,
      status: "open" as const,
    },
    {
      id: "2",
      name: "Weekly Challenge",
      prizePool: 5000,
      status: "starting-soon" as const,
    },
  ];

  return (
    <main className="min-h-screen relative right-4 bg-[#070B1D]">
      {/* Top left Community title */}
      <h1 className="absolute top-5 left-5 text-white font-semibold text-3xl">
        Community
      </h1>

      <div className="w-full flex justify-end">
        <div className="max-w-2xl w-full">
          <TournamentList tournaments={tournaments} />
        </div>
      </div>
    </main>
  );
};

export default Community;
