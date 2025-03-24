import React from "react";
import { Trophy } from "lucide-react"; // Using lucide-react for the trophy icon

// Define tournament type
interface Tournament {
  id: number;
  name: string;
  prizePool: number;
  status: "Registrations Open" | "Starting Soon" | "In Progress" | "Closed";
}

// Props for the component
interface TournamentsListProps {
  tournaments: Tournament[];
}

const statusColors: Record<Tournament["status"], string> = {
  "Registrations Open": "text-green-500 bg-green-100",
  "Starting Soon": "text-yellow-600 bg-yellow-100",
  "In Progress": "text-blue-500 bg-blue-100",
  "Closed": "text-gray-500 bg-gray-100",
};

const TournamentsList: React.FC<TournamentsListProps> = ({ tournaments }) => {
  return (
    <div className="bg-[#0A1F44] text-white p-6 rounded-2xl shadow-lg w-full max-w-md mx-auto">
      {/* Title Section */}
      <div className="flex items-center gap-2 mb-4">
        <Trophy className="text-yellow-400" size={24} />
        <h2 className="text-xl font-semibold">Tournaments</h2>
      </div>

      {/* Tournament List */}
      {tournaments.length > 0 ? (
        <ul className="space-y-4">
          {tournaments.map((tournament) => (
            <li
              key={tournament.id}
              className="p-4 bg-[#142850] rounded-xl shadow flex flex-col"
            >
              <div className="flex justify-between">
                <h3 className="text-lg font-medium">{tournament.name}</h3>
                <span className="text-gray-300 font-medium">
                  ${tournament.prizePool.toLocaleString()}
                </span>
              </div>
              <span
                className={`mt-2 px-3 py-1 text-sm font-semibold rounded-md ${statusColors[tournament.status]}`}
              >
                {tournament.status}
              </span>
            </li>
          ))}
        </ul>
      ) : (
        <p className="text-gray-400 text-center">No tournaments available</p>
      )}
    </div>
  );
};

export default TournamentsList;
