"use client"
import Image from "next/image";
import React from "react";

interface GameCardProp {
  title: string;
  subTitle: string;
  image: string;
  discount?: number;
  genre: string;
  starRating: number;
  currentPrice: number;
  originalPrice?: number;
  author: string;
  onPlay: () => void;
}

function GameCard({
  title,
  subTitle,
  image,
  discount,
  genre,
  starRating,
  currentPrice,
  originalPrice,
  author,
  onPlay,
}: GameCardProp) {
  return (
    <div className="w-[19.88rem] h-[23.29rem] rounded-[0.5rem] border-2 border-[#16213E] bg-[#121417] overflow-hidden">
      <div className="w-full h-1/2 relative">
        <Image
          src={image || "/pngs/fallback-placeholder.png"}
          alt={title}
          width={314}
          height={176.63}
          quality={90}
          className="w-full h-full object-center hover:scale-110 duration-700 transition-transform"
        />
        <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-t from-[#000000CC] via-[#00000000] to-[#00000000] hover:bg-none duration-700 transition-colors"></div>

        {discount && (
          <div className="w-[3.0625rem] h-[1.625rem] rounded-full bg-[#FF0080] font-bold text-xs absolute top-1.5 left-1.5 flex justify-center items-center">
            -{discount}%
          </div>
        )}

        <div className="w-[5.25rem] h-[1.375rem] rounded-full bg-[#050508CC] border border-[#00D4FF4D] text-[#00D4FF] text-xs font-semibold flex justify-center items-center absolute top-1.5 right-1.5">
          {genre}
        </div>
        <div className="w-[3.4rem] h-[1.75rem] px-[0.1875rem] py-[0.25rem] font-bold text-sm flex justify-center items-center gap-1.5 bg-[#000000B2] rounded-[0.5rem] absolute bottom-2 right-1.5">
          <Image
            src="/svgs/star.svg"
            alt="ratings"
            width={10}
            height={9.51}
            quality={90}
          />
          {starRating}
        </div>
      </div>

      <div className="w-full h-1/2 bg-gradient-to-b from-[#1A1A2E] to-[#050508] p-[1.25rem]">
        <p className="text-white font-bold text-[1.125rem]">{title}</p>
        <p className="text-[#9CA3AF] text-sm pt-1.5">{subTitle}</p>
        <p className="text-[#6B7280] text-xs pt-1.5 pb-2.5">by {author}</p>

        <div className="w-full flex justify-between items-center">
            <div className="flex items-center gap-1.5">
                <p className="font-bold text-[1.25rem] text-[#00D4FF]">${currentPrice}</p>
                {originalPrice && (<p className="text-sm text-[#6B7280] line-through">${originalPrice}</p>)}
            </div>

            <button className='w-[3.75rem] h-[2.25rem] rounded-[0.33rem] bg-gradient-to-br from-[#00D4FF] to-[#A855F7] text-sm text-[#0D0F11] font-bold cursor-pointer' onClick={() => onPlay()}>Play</button>
        </div>
      </div>
    </div>
  );
}

export default GameCard;
