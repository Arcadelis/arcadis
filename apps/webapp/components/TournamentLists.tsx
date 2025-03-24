import { Trophy } from "lucide-react";
import "@fontsource/inter";

type TournamentStatus = "open" | "starting-soon" | "in-progress" | "closed";

interface Tournament {
  id: string;
  name: string;
  prizePool: number;
  status: TournamentStatus;
}

interface TournamentListProps {
  tournaments: Tournament[];
}

export default function TournamentList({ tournaments }: TournamentListProps) {
  const getStatusColor = (status: TournamentStatus) => {
    switch (status) {
      case "open":
        return "text-[#22C55E]";
      case "starting-soon":
        return "text-[#EAB308]";
      case "in-progress":
        return "text-blue-400";
      case "closed":
        return "text-gray-400";
      default:
        return "text-gray-400";
    }
  };

  const getStatusText = (status: TournamentStatus) => {
    switch (status) {
      case "open":
        return "Registrations Open";
      case "starting-soon":
        return "Starting Soon";
      case "in-progress":
        return "In Progress";
      case "closed":
        return "Closed";
      default:
        return "Unknown";
    }
  };

  const formatPrizePool = (amount: number) => {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
      maximumFractionDigits: 0,
    }).format(amount);
  };

  return (
    <div className="rounded-lg w-4/5 bg-[#2A2B5B] p-6 text-white ml-auto mr-28 mt-40 border-gray-700 border shadow-[0px_1px_3px_0px_rgba(0,0,0,0.1)]">
      <div className="mb-6 flex items-center gap-2 justify-start">
        <Trophy className="h-6 w-6 text-emerald-400" />
        <h2 className="font-inter font-semibold text-[15.5px] leading-[16px] tracking-[-0.4px] align-middle">
          Tournaments
        </h2>
      </div>

      {tournaments.length === 0 ? (
        <div className="rounded-md bg-[#1A1B4B] p-4 text-center text-gray-400">
          No tournaments available at the moment.
        </div>
      ) : (
        <div className="space-y-4">
          {tournaments.map((tournament) => (
            <div
              key={tournament.id}
              className="rounded-md bg-[#141a3a] p-4 border border-gray-700 shadow-[0px_1px_2px_-1px_rgba(0,0,0,0.1)]"
            >
              <h3 className="font-inter font-bold text-[15.63px] leading-[24px] tracking-[0%] align-middle">{tournament.name}</h3>
              <div className="mt-1 flex flex-col">
                <p className="font-inter font-normal text-[15.38px] leading-[24px] tracking-[0%] align-middle text-[#9CA3AF]">
                  Prize Pool: {formatPrizePool(tournament.prizePool)}
                </p>
                <p
                  className={`${getStatusColor(tournament.status)} font-medium`}
                >
                  {getStatusText(tournament.status)}
                </p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
