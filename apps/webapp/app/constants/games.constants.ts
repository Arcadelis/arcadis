export interface Game {
  id: string;
  slug: string;
  rating: number;
  statusLabel: string;
  statusColor: string;
  bannerImage: string;
  gameIcon: string;
  title: string;
  developer: string;
  description: string;
  xLink: string;
  tgLink: string;
  webLink: string;
  tags: string[];
  stats: {
    players: string;
    community: string;
  };
  hasWebsiteLink: boolean;
  // Additional fields for game details page
  studioName?: string;
  stellarImage?: string;
  bannerTagline?: string;
  introductionText?: string;
}

export const GAMES_DATA: Game[] = [
  {
    id: "1",
    slug: "zkops",
    rating: 9,
    statusLabel: "Active",
    statusColor: "#10B981",
    bannerImage: "/image 2.png",
    gameIcon: "/gameAvater.png",
    title: "zKops",
    developer: "Korporations",
    description:
      "zKops is an engaging puzzle game that puts players' strategic thinking to the test. Set within a dynamic",
    xLink: "https://x.com/zkops",
    tgLink: "https://t.me/zkops",
    webLink: "https://zkops.game",
    tags: ["PUZZLE", "STRATEGIC"],
    stats: {
      players: "500,000+",
      community: "1.2",
    },
    hasWebsiteLink: true,
    studioName: "Korporations",
    stellarImage: "/gamer-img.png",
    bannerTagline: "STRATEGIC PUZZLE MASTERPIECE!",
    introductionText:
      "zKops is an engaging puzzle game that puts players' strategic thinking to the test. Set within a dynamic world where every decision matters, players must solve increasingly complex challenges using logic and creativity.",
  },
  {
    id: "2",
    slug: "stellar-odyssey",
    rating: 8,
    statusLabel: "Active",
    statusColor: "#10B981",
    bannerImage: "/stellar-odyssey-banner.png",
    gameIcon: "/gameAvater.png",
    title: "Stellar Odyssey",
    developer: "Cosmic Games Studio",
    description:
      "Embark on an interstellar journey across a procedurally generated universe. Discover ancient artifacts and engage with alien civilizations.",
    xLink: "https://x.com/stellarodyssey",
    tgLink: "https://t.me/stellarodyssey",
    webLink: "https://stellarodyssey.game",
    tags: ["ADVENTURE", "SPACE"],
    stats: {
      players: "300,000+",
      community: "0.8",
    },
    hasWebsiteLink: true,
    studioName: "Cosmic Games Studio",
    stellarImage: "/gamer-img.png",
    bannerTagline: "EMBARK ON A COSMIC ADVENTURE!",
    introductionText:
      "Embark on an interstellar journey across a procedurally generated universe. As a deep space explorer, you'll discover ancient artifacts, engage in diplomatic relations with alien civilizations, and uncover the mysteries of a dying universe.",
  },
  {
    id: "3",
    slug: "crypto-quest",
    rating: 7,
    statusLabel: "Upcoming",
    statusColor: "#F59E0B",
    bannerImage: "/image 2.png",
    gameIcon: "/gameAvater.png",
    title: "Crypto Quest",
    developer: "Blockchain Studios",
    description:
      "A revolutionary blockchain-based RPG where players can truly own their in-game assets and trade them freely.",
    xLink: "https://x.com/cryptoqueest",
    tgLink: "https://t.me/cryptoqueest",
    webLink: "https://cryptoqueest.game",
    tags: ["RPG", "BLOCKCHAIN"],
    stats: {
      players: "200,000+",
      community: "0.5",
    },
    hasWebsiteLink: true,
    studioName: "Blockchain Studios",
    stellarImage: "/gamer-img.png",
    bannerTagline: "OWN YOUR ADVENTURE!",
    introductionText:
      "A revolutionary blockchain-based RPG where players can truly own their in-game assets and trade them freely. Experience a world where every item, character, and achievement has real value.",
  },
  {
    id: "4",
    slug: "neon-racing",
    rating: 8,
    statusLabel: "Active",
    statusColor: "#10B981",
    bannerImage: "/image 2.png",
    gameIcon: "/gameAvater.png",
    title: "Neon Racing",
    developer: "Speed Demons",
    description:
      "High-octane racing through neon-lit cyberpunk cities with stunning visuals and intense competition.",
    xLink: "https://x.com/neonracing",
    tgLink: "https://t.me/neonracing",
    webLink: "https://neonracing.game",
    tags: ["RACING", "CYBERPUNK"],
    stats: {
      players: "400,000+",
      community: "1.0",
    },
    hasWebsiteLink: true,
    studioName: "Speed Demons",
    stellarImage: "/gamer-img.png",
    bannerTagline: "RACE INTO THE FUTURE!",
    introductionText:
      "High-octane racing through neon-lit cyberpunk cities with stunning visuals and intense competition. Experience the thrill of high-speed chases in a dystopian future where every race could be your last.",
  },
  {
    id: "5",
    slug: "mystic-realms",
    rating: 9,
    statusLabel: "Active",
    statusColor: "#10B981",
    bannerImage: "/image 2.png",
    gameIcon: "/gameAvater.png",
    title: "Mystic Realms",
    developer: "Fantasy Forge",
    description:
      "An epic fantasy MMORPG with deep lore, magical combat, and endless exploration possibilities.",
    xLink: "https://x.com/mysticrealms",
    tgLink: "https://t.me/mysticrealms",
    webLink: "https://mysticrealms.game",
    tags: ["MMORPG", "FANTASY"],
    stats: {
      players: "600,000+",
      community: "1.5",
    },
    hasWebsiteLink: true,
    studioName: "Fantasy Forge",
    stellarImage: "/gamer-img.png",
    bannerTagline: "ENTER THE REALM OF MAGIC!",
    introductionText:
      "An epic fantasy MMORPG with deep lore, magical combat, and endless exploration possibilities. Forge alliances, master powerful spells, and become a legend in a world where magic and technology collide.",
  },
  {
    id: "6",
    slug: "quantum-shift",
    rating: 8,
    statusLabel: "Upcoming",
    statusColor: "#F59E0B",
    bannerImage: "/image 2.png",
    gameIcon: "/gameAvater.png",
    title: "Quantum Shift",
    developer: "Quantum Games",
    description:
      "A mind-bending puzzle platformer where players manipulate quantum mechanics to solve impossible challenges.",
    xLink: "https://x.com/quantumshift",
    tgLink: "https://t.me/quantumshift",
    webLink: "https://quantumshift.game",
    tags: ["PUZZLE", "QUANTUM"],
    stats: {
      players: "150,000+",
      community: "0.4",
    },
    hasWebsiteLink: true,
    studioName: "Quantum Games",
    stellarImage: "/gamer-img.png",
    bannerTagline: "SHIFT YOUR REALITY!",
    introductionText:
      "A mind-bending puzzle platformer where players manipulate quantum mechanics to solve impossible challenges. Experience physics like never before in this innovative take on the puzzle genre.",
  },
];

export const getGameBySlug = (slug: string): Game | undefined => {
  return GAMES_DATA.find((game) => game.slug === slug);
};

export const getAllGames = (): Game[] => {
  return GAMES_DATA;
};
