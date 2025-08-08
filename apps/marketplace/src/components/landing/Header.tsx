import Image from "next/image";
import React from "react";

function Header() {
  return (
    <div className="w-full h-[34.8125rem] bg-gradient-to-r from-[#00D4FF1A] to-[#A855F71A] flex flex-col justify-center items-center">
      <div className="flex items-center gap-6">
        <Image
          src="/svgs/logo-icon.svg"
          alt="arcadis_icon"
          width={80}
          height={80}
          quality={90}
          className="w-[5rem] h-[5rem]"
        />
        <div>
          <p className="bg-gradient-to-r from-[#00D4FF] via-[#FFFFFF] to-[#A855F7] text-transparent bg-clip-text text-[4.5rem] font-black">
            ARCADIS
          </p>
          <p className="text-[#00D4FF] text-[1.5rem] font-semibold">
            THE OPEN MARKETPLACE FOR INDIE GAMES
          </p>
        </div>
      </div>
      <p className="text-[#D1D5DB] text-[1.25rem] text-center pt-4">
        Discover the next generation of indie gaming. Support independent
        creators, find
        <br />
        hidden gems, and experience games that push boundaries.
      </p>

      <div className="pt-8 flex items-center gap-7">
        <button className="w-[15.5rem] h-[2.75rem] rounded-[0.375rem] flex justify-center items-center gap-x-4 text-[#0D0F11] font-medium text-[1.125rem] bg-gradient-to-br from-[#00D4FF] to-[#A855F7] cursor-pointer">
            <Image src="/svgs/explore_games.svg" alt="explore_games" width={9.33} height={12} quality={90} />
            <span>EXPLORE GAMES</span>
        </button>
        <button className="w-[14.75rem] h-[2.75rem] rounded-[0.375rem] flex justify-center items-center gap-x-4 text-[#A855F7] font-medium text-[1.125rem] bg-[#0D0F11] border-[2px] border-[#A855F7] cursor-pointer">
            <Image src="/svgs/start_selling.svg" alt="start_selling" width={12} height={13.34} quality={90} />
            <span>START SELLING</span>
        </button>
      </div>
    </div>
  );
}

export default Header;
