"use client";

import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import React from "react";

const navItems = [
  { href: "/games", label: "Browse Games" },
  { href: "/", label: "Developer Hub" },
];

function Navbar() {
  const pathname = usePathname();

  const isActive = (href: string) => {
    return pathname === href;
  };

  return (
    <nav className="w-full bg-[#050508CC] h-[5.3rem] fixed top-0 left-0 px-[3.625rem] flex justify-between items-center z-50 backdrop-blur-sm">
      <Image src="/svgs/logo.svg" alt="arcadis-marketplace" width={187} height={52} quality={90} />

      <div className="flex items-center gap-x-[1.5rem]">
        {navItems.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            aria-current={isActive(item.href) ? "page" : undefined}
            className="text-base"
          >
            <span
              className={
                isActive(item.href)
                  ? // text with active gradient
                  "bg-gradient-to-br from-[#00D4FF] to-[#A855F7] bg-clip-text text-transparent font-bold"
                  : // normal text
                  "text-[#94A3B8] hover:text-white transition-colors"
              }
            >
              {item.label}
            </span>
          </Link>
        ))}

        <Link
          href="/sign-in"
          className="w-[4.9375rem] h-[2.5rem] rounded-[0.375rem] bg-[#0D0F11] border border-[#16213E] text-sm font-medium cursor-pointer flex justify-center items-center"
        >
          Sign In
        </Link>

        <Link
          href="/sign-up"
          className="w-[5.1875rem] h-[2.5rem] rounded-[0.375rem] bg-gradient-to-br from-[#00D4FF] to-[#A855F7] text-sm text-[#0D0F11] font-medium cursor-pointer flex justify-center items-center"
        >
          Sign Up
        </Link>
      </div>
    </nav>
  );
}

export default Navbar;
