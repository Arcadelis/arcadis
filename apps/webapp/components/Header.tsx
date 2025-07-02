"use client"

import Link from "next/link"
import { cn } from "@/lib/utils"
import * as DropdownMenu from "@radix-ui/react-dropdown-menu"
import { Menu, X, ChevronDown, ChevronUp } from "lucide-react"
import { useMenu } from "@/context/MenuContext"
import { WalletConnectButton } from "@/components/WalletConnectButton"

const navigation = [
  { name: "Home", href: "/" },
  { name: "Games", href: "/games" },
  { name: "Governance", href: "/governance" }, // Added governance link
  { name: "News", href: "/news" },
  { name: "Community", href: "/community" },
]

const awardOptions = [
  { name: "Hall of Fame", href: "/awards/halloffame" },
  { name: "Monthly Award", href: "/awards/monthlyaward" },
  { name: "Nominations", href: "/awards/nominations" },
]

export function Header() {
  const { isMenuOpen, setIsMenuOpen, isAwardsOpen, setIsAwardsOpen } = useMenu()

  return (
    <header className="fixed top-0 left-0 right-0 z-50 bg-[#13163A] backdrop-blur-sm border-b border-white/10 font-inter">
      <nav className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8" aria-label="Top">
        <div className="flex h-16 items-center justify-between">
          {/* Mobile Menu Button (Left) */}
          <div className="flex md:hidden items-center w-[100px]">
            <DropdownMenu.Root open={isMenuOpen} onOpenChange={setIsMenuOpen}>
              <DropdownMenu.Trigger asChild>
                <button className="p-2 text-white hover:bg-white/5 rounded-md transition-colors">
                  {isMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
                </button>
              </DropdownMenu.Trigger>
              <DropdownMenu.Portal>
                <DropdownMenu.Content
                  className="w-screen bg-[#13163A] py-4 shadow-lg border-b border-white/10"
                  sideOffset={16}
                  align="start"
                >
                  <div className="px-4 space-y-2">
                    {navigation.map((item) => (
                      <DropdownMenu.Item key={item.name}>
                        <Link
                          href={item.href}
                          className="flex w-full items-center px-3 py-2 text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 hover:bg-white/5 rounded-md transition-colors tracking-[0%]"
                        >
                          {item.name}
                        </Link>
                      </DropdownMenu.Item>
                    ))}
                    {/* Awards Menu Mobile */}
                    <div className="space-y-1">
                      <button
                        onClick={() => setIsAwardsOpen(!isAwardsOpen)}
                        className="flex w-full items-center justify-between px-3 py-2 text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 hover:bg-white/5 rounded-md transition-colors tracking-[0%]"
                      >
                        <span>Awards</span>
                        {isAwardsOpen ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
                      </button>
                      {isAwardsOpen && (
                        <div className="pl-4 space-y-1 border-l border-white/10 ml-3">
                          {awardOptions.map((option) => (
                            <DropdownMenu.Item key={option.name}>
                              <Link
                                href={option.href}
                                className="flex w-full items-center px-3 py-2 text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 hover:bg-white/5 rounded-md transition-colors tracking-[0%]"
                              >
                                {option.name}
                              </Link>
                            </DropdownMenu.Item>
                          ))}
                        </div>
                      )}
                    </div>
                  </div>
                </DropdownMenu.Content>
              </DropdownMenu.Portal>
            </DropdownMenu.Root>
          </div>

          {/* Logo */}
          <div className="flex items-center w-[100px] md:w-auto justify-center md:justify-start">
            <Link href="/" className="text-2xl font-bold text-white">
              Arcadelis
            </Link>
          </div>

          {/* Desktop Navigation */}
          <div className="hidden md:flex md:items-center md:space-x-8">
            {navigation.map((item) => (
              <Link
                key={item.name}
                href={item.href}
                className={cn(
                  "text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 transition-colors tracking-[0%] text-center",
                  "px-3 py-2 rounded-md",
                )}
              >
                {item.name}
              </Link>
            ))}
            {/* Awards Dropdown Desktop */}
            <DropdownMenu.Root>
              <DropdownMenu.Trigger asChild>
                <button className="flex items-center gap-1 text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 transition-colors tracking-[0%] text-center px-3 py-2 rounded-md">
                  <span>Awards</span>
                  <span className="text-xs">▾</span>
                </button>
              </DropdownMenu.Trigger>
              <DropdownMenu.Portal>
                <DropdownMenu.Content
                  className="min-w-[160px] bg-[#13163A] rounded-lg p-1 shadow-lg border border-white/10"
                  sideOffset={5}
                  align="center"
                >
                  {awardOptions.map((option) => (
                    <DropdownMenu.Item key={option.name}>
                      <Link
                        href={option.href}
                        className="flex w-full items-center px-3 py-2 text-[13.67px] leading-[20px] font-medium text-white hover:text-white/90 hover:bg-white/5 rounded-md transition-colors tracking-[0%] text-center"
                      >
                        {option.name}
                      </Link>
                    </DropdownMenu.Item>
                  ))}
                </DropdownMenu.Content>
              </DropdownMenu.Portal>
            </DropdownMenu.Root>
          </div>

          {/* Connect Button */}
          <div className="flex items-center w-[100px] justify-end">
            <WalletConnectButton />
          </div>
        </div>
      </nav>
    </header>
  )
}
