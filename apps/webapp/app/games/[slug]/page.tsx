import { notFound } from "next/navigation";
import GameDetailsLayout from "@/components/GameDetails/GameDetailsLayout";
import { getGameBySlug } from "@/app/constants/games.constants";

interface GameDetailsPageProps {
  params: {
    slug: string;
  };
}

export default function GameDetailsPage({ params }: GameDetailsPageProps) {
  const game = getGameBySlug(params.slug);

  if (!game) {
    notFound();
  }

  return (
    <>
      <GameDetailsLayout
        gameTitle={game.title}
        studioName={game.studioName || game.developer}
        bannerImage={game.bannerImage}
        stellarImage={game.stellarImage || "/gamer-img.png"}
        bannerTagline={
          game.bannerTagline ||
          `${game.title.toUpperCase()} - ${game.tags.join(" & ")}`
        }
        introductionText={game.introductionText || game.description}
        initialActiveTab="Overview"
      >
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6 lg:gap-8">
          <div className="lg:col-span-2 w-full lg:w-[895px] lg:max-w-[895px]">
            {/* Game Stats Section */}
            <div className="bg-[#1a1b4b] rounded-lg p-4 sm:p-6 mb-4 sm:mb-6 lg:mb-8">
              <h3 className="text-lg sm:text-xl font-bold mb-2">
                Game Statistics
              </h3>
              <p className="text-[#9CA3AF] font-normal text-xs sm:text-sm mb-4">
                Current game metrics and performance
              </p>

              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div className="text-center">
                  <div className="text-2xl font-bold text-white">
                    {game.rating}/10
                  </div>
                  <div className="text-sm text-[#9CA3AF]">Rating</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-white">
                    {game.stats.players}
                  </div>
                  <div className="text-sm text-[#9CA3AF]">Players</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-white">
                    {game.stats.community}M+
                  </div>
                  <div className="text-sm text-[#9CA3AF]">Community</div>
                </div>
                <div className="text-center">
                  <div className="text-2xl font-bold text-white">
                    {game.tags.length}
                  </div>
                  <div className="text-sm text-[#9CA3AF]">Categories</div>
                </div>
              </div>
            </div>

            {/* Game Tags Section */}
            <div className="bg-[#1a1b4b] rounded-lg p-4 sm:p-6 mb-4 sm:mb-6 lg:mb-8">
              <h3 className="text-lg sm:text-xl font-bold mb-2">
                Game Categories
              </h3>
              <div className="flex flex-wrap gap-2">
                {game.tags.map((tag) => (
                  <span
                    key={tag}
                    className="px-3 py-1 rounded-[6px] text-xs font-semibold bg-[#064E3B] text-[#34D399]"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            </div>

            {/* Recent Transactions Section */}
            <div className="bg-[#1a1b4b] rounded-lg p-4 sm:p-6 mb-4 sm:mb-6 lg:mb-8">
              <h3 className="text-lg sm:text-xl font-bold mb-2">
                Recent Transactions
              </h3>
              <p className="text-[#9CA3AF] font-normal text-xs sm:text-sm mb-4">
                Latest transactions on the network
              </p>

              <div className="overflow-x-auto -mx-4 sm:mx-0">
                <div className="min-w-max sm:min-w-0 px-4 sm:px-0">
                  <table className="w-full">
                    <thead>
                      <tr className="text-left text-gray-400 text-xs sm:text-sm border-b border-[#2a2b5b]">
                        <th className="pb-2 font-medium text-[#9CA3AF]">
                          Hash
                        </th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">
                          Block
                        </th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">
                          Type
                        </th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">
                          Status
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr className="">
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">
                          <div className="text-blue-400 truncate max-w-[120px] sm:max-w-xs text-[#60A5FA]">
                            0x63cf80c89e9bd0655688854aed482f09434f256b0439f44fc2beb620cf625815
                          </div>
                        </td>
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">
                          293739
                        </td>
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">
                          INVOKE
                        </td>
                        <td className="py-2 sm:py-3">
                          <span className="bg-green-500/20 text-green-400 text-[10px] sm:text-xs font-semibold px-1.5 sm:px-2 py-0.5 sm:py-1 rounded">
                            ACCEPTED_ON_L1
                          </span>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>

          {/* Sidebar */}
          <div className="lg:col-span-1">
            <div className="bg-[#1a1b4b] rounded-lg p-4 sm:p-6">
              <h3 className="text-lg sm:text-xl font-bold mb-4">Game Links</h3>
              <div className="space-y-3">
                <a
                  href={game.xLink}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-3 p-3 bg-[#2a2b5b] rounded-lg hover:bg-[#3a3b6b] transition-colors"
                >
                  <div className="w-8 h-8 bg-white rounded-[6px] flex justify-center items-center">
                    <span className="text-black font-bold text-sm">X</span>
                  </div>
                  <span className="text-white">Follow on X</span>
                </a>
                <a
                  href={game.tgLink}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-3 p-3 bg-[#2a2b5b] rounded-lg hover:bg-[#3a3b6b] transition-colors"
                >
                  <div className="w-8 h-8 bg-white rounded-[6px] flex justify-center items-center">
                    <span className="text-black font-bold text-sm">TG</span>
                  </div>
                  <span className="text-white">Join Telegram</span>
                </a>
                {game.hasWebsiteLink && (
                  <a
                    href={game.webLink}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center gap-3 p-3 bg-[#2a2b5b] rounded-lg hover:bg-[#3a3b6b] transition-colors"
                  >
                    <div className="w-8 h-8 bg-white rounded-[6px] flex justify-center items-center">
                      <span className="text-black font-bold text-sm">🌐</span>
                    </div>
                    <span className="text-white">Visit Website</span>
                  </a>
                )}
              </div>
            </div>
          </div>
        </div>
      </GameDetailsLayout>
    </>
  );
}
