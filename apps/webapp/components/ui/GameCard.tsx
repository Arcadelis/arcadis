"use client";

import { Globe } from "lucide-react";
import Image from "next/image";
import { FaTelegramPlane } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";
import { useRouter } from "next/navigation";

const Badge = ({
  children,
  className,
  style,
}: {
  children: React.ReactNode;
  className?: string;
  style?: React.CSSProperties;
}) => (
  <span
    className={`px-3 py-1 rounded-[6px] text-xs font-semibold ${className}`}
    style={style}
  >
    {children}
  </span>
);

const Card = ({
  children,
  className,
  onClick,
}: {
  children: React.ReactNode;
  className?: string;
  onClick?: (e: React.MouseEvent) => void;
}) => (
  <div
    className={`bg-[#BFDBFE1A] border-[1px] border-[#F3F4F626] p-5 rounded-xl min-w-[330px] md:w-[403.11px] h-auto md:h-[603.87px] text-white ${className}`}
    onClick={onClick}
  >
    {children}
  </div>
);

const CardContent = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => <div className={`mt-4 ${className}`}>{children}</div>;

interface GameCardProps {
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
  stats: { players: string; community: string };
  hasWebsiteLink: boolean;
}

const GameCard: React.FC<GameCardProps> = ({
  slug,
  rating,
  statusLabel,
  statusColor,
  bannerImage,
  gameIcon,
  title,
  developer,
  description,
  xLink,
  tgLink,
  webLink,
  tags,
  stats,
  hasWebsiteLink,
}) => {
  const router = useRouter();

  const handleCardClick = (e: React.MouseEvent) => {
    // Prevent navigation if clicking on social links
    if ((e.target as HTMLElement).closest("a")) {
      return;
    }
    router.push(`/games/${slug}`);
  };
  return (
    <Card
      className="cursor-pointer hover:bg-[#BFDBFE26] transition-colors"
      onClick={handleCardClick}
    >
      <div className="relative font-sans flex items-center justify-center mb-5 h-[33%]">
        <Image
          height={100}
          width={100}
          src={bannerImage}
          alt="Game Banner"
          className="rounded-lg w-full h-full object-cover"
        />
        <Badge
          className="absolute top-2 left-2 text-white"
          style={{ backgroundColor: statusColor }}
        >
          {statusLabel}
        </Badge>
      </div>

      <CardContent>
        <div className="flex items-start gap-4">
          <Image
            src={gameIcon}
            alt="Game Avatar"
            height={100}
            width={100}
            className=" w-10 h-10 md:w-16 md:h-16 rounded-lg object-cover"
          />
          <div className="flex-1 flex flex-col gap-4 mb-9">
            <h3 className="text-base md:text-[19.06px] text-white font-bold">
              {title}
            </h3>
            <p className="text-[#9CA3AF] font-normal text-xs md:text-[13.63px] ">
              {developer}
            </p>
          </div>
          <div className="bg-[#10B981] shadow-[0px_1px_3px_0px_#0000001A] text-white w-6 h-6 md:w-8 md:h-8 flex items-center justify-center rounded-md text-sm md:text-lg font-bold">
            {rating}
          </div>
        </div>

        <p className="   text-xs md:text-[13.67px] text-[#D1D5DB] font-normal mt-3 mb-6">
          {description}
        </p>

        <div className="flex flex-col items-start gap-2 w-full justify-between">
          <div className="w-full flex justify-between text-gray-400 text-sm">
            <span className="text-[#9CA3AF] text-xs md:text-[13.56px] font-normal">
              Players
            </span>
            <span className="text-white text-sm md:text-[15.5px] font-medium">
              {stats.players}
            </span>
          </div>
          <div className="w-full flex justify-between text-gray-400 text-sm mt-1">
            <span className="text-[#9CA3AF] text-xs md:text-[13.56px] font-normal">
              Community
            </span>
            <span className="text-white ext-sm md:text-[15.5px]font-medium">
              {stats.community}M+
            </span>
          </div>
        </div>

        <div className="flex gap-2 mt-6">
          {tags.map((tag) => (
            <Badge key={tag} className="bg-[#064E3B] text-[#34D399]">
              {tag}
            </Badge>
          ))}
        </div>

        <div className="flex items-center gap-4 mt-9 text-[#0A0A0A] text-xl">
          <a
            href={xLink}
            className="w-9 h-9 bg-white rounded-[6px] flex justify-center items-center"
          >
            <FaXTwitter className="cursor-pointer" />
          </a>
          <a
            href={tgLink}
            className="w-9 h-9 bg-white rounded-[6px] flex justify-center items-center"
          >
            <FaTelegramPlane className="cursor-pointer" />
          </a>
          {hasWebsiteLink && (
            <a
              href={webLink}
              className="ml-auto w-9 h-9 bg-white rounded-[6px] flex justify-center items-center"
            >
              <Globe className="cursor-pointer" />
            </a>
          )}
        </div>
      </CardContent>
    </Card>
  );
};

export default GameCard;
