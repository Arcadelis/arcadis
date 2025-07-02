import React from "react";
import GameCard from "../ui/GameCard";
import { getAllGames } from "@/app/constants/games.constants";

const GameCatalog = () => {
  const games = getAllGames();

  return (
    <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-6 py-6 ">
      {games.map((game) => (
        <GameCard key={game.id} {...game} />
      ))}
    </div>
  );
};

export default GameCatalog;
