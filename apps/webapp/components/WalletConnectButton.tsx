"use client"

import { useState } from "react"
import { Wallet, LogOut } from "lucide-react"
import { Button } from "@/components/ui/button"
import { useWallet } from "@/app/hooks/useWallet"
import { cn } from "@/lib/utils"

export function WalletConnectButton() {
  const { connectWallet, disconnectWallet, isConnected, walletAddress, isConnecting } = useWallet()

  const [errorMessage, setErrorMessage] = useState<string | null>(null)

  const handleConnect = async () => {
    setErrorMessage(null)
    const result = await connectWallet()
    if (!result.success && result.error) {
      setErrorMessage(result.error)
      console.error(result.error)
    }
  }

  const handleDisconnect = async () => {
    setErrorMessage(null)
    const result = await disconnectWallet()
    if (!result.success && result.error) {
      setErrorMessage(result.error)
      console.error(result.error)
    }
  }

  const truncateAddress = (addr: string) => `${addr.slice(0, 6)}…${addr.slice(-4)}`

  if (isConnected && walletAddress) {
    return (
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="default"
          className="rounded-full px-4 py-2 text-[13.67px] leading-[20px] gap-2 bg-transparent"
          onClick={handleDisconnect}
        >
          <span>{truncateAddress(walletAddress)}</span>
          <LogOut className="h-3 w-3" />
        </Button>
      </div>
    )
  }

  return (
    <Button
      onClick={handleConnect}
      disabled={isConnecting}
      className={cn(
        "bg-gradient-to-r from-[#3B82F6] to-[#0E7490]",
        "text-white",
        "rounded-full",
        "px-4 py-2",
        "text-[13.67px] leading-[20px]",
        "transition-all duration-300 hover:opacity-90",
        "gap-2",
      )}
    >
      <Wallet className="h-4 w-4" />
      {isConnecting ? "Connecting…" : "Connect Wallet"}
    </Button>
  )
}
