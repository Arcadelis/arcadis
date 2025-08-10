import Image from "next/image";
import Link from "next/link";
import React from "react";

function Footer() {
  return (
    <footer className="w-full mt-2 bg-[#050508] border-t border-t-[#16213E] py-[4.0625rem] px-[3.5rem]">
      <div className="w-full px-[2rem]">
        <div className="w-full border-b border-b-[#16213E] flex flex-col md:flex-row gap-x-[2rem] pb-[3rem]">
          <ul className="w-1/4">
            <li className="mb-[1.5rem]">
              <Link href="#" className="flex items-center gap-1.5">
                <Image
                  src="/svgs/logo-icon.svg"
                  alt="arcadis_icon"
                  width={26.67}
                  height={18.67}
                  quality={90}
                  className="w-[2.1rem] h-[1.57rem]"
                />
                <p className="bg-gradient-to-r from-[#00D4FF] to-[#A855F7] text-transparent bg-clip-text text-[1.5rem] font-bold">
                  ARCADIS
                </p>
              </Link>
            </li>
            <li className="text-[1.125rem] text-[#9CA3AF]">
              The ultimate destination for indie
              <br />
              gaming excellence.
            </li>
          </ul>
          <ul className="w-1/4 flex flex-col gap-y-[0.33rem]">
            <li className="text-[1.125rem] font-bold text-[#00D4FF] mb-[1.7292rem]">FOR GAMERS</li>
            <li className="text-[#9CA3AF]"><Link href="#">Browse Games</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Wishlist</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Reviews</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Community</Link></li>
          </ul>
          <ul className="w-1/4 flex flex-col gap-y-[0.33rem]">
            <li className="text-[1.125rem] font-bold text-[#A855F7] mb-[1.7292rem]">FOR DEVELOPERS</li>
            <li className="text-[#9CA3AF]"><Link href="#">Developer Hub</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Sell Your Game</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Resources</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Analytics</Link></li>
          </ul>
          <ul className="w-1/4 flex flex-col gap-y-[0.33rem]">
            <li className="text-[1.125rem] font-bold text-[#00FF88] mb-[1.7292rem]">SUPPORT</li>
            <li className="text-[#9CA3AF]"><Link href="#">Help Center</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Contact</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Terms</Link></li>
            <li className="text-[#9CA3AF]"><Link href="#">Privacy</Link></li>
          </ul>
        </div>
        <p className="text-[1.125rem] text-[#6B7280] text-center mt-[2.0625rem]">
          © 2024 ARCADIS MARKETPLACE. ALL RIGHTS RESERVED.
        </p>
      </div>
    </footer>
  );
}

export default Footer;
