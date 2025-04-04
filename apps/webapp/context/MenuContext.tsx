'use client'

import { createContext, useContext, useState } from 'react'

type MenuContextType = {
  isMenuOpen: boolean
  setIsMenuOpen: (open: boolean) => void
  isAwardsOpen: boolean
  setIsAwardsOpen: (open: boolean) => void
}

const MenuContext = createContext<MenuContextType | undefined>(undefined)

export function MenuProvider({ children }: { children: React.ReactNode }) {
  const [isMenuOpen, setIsMenuOpen] = useState(false)
  const [isAwardsOpen, setIsAwardsOpen] = useState(false)

  return (
    <MenuContext.Provider value={{ isMenuOpen, setIsMenuOpen, isAwardsOpen, setIsAwardsOpen }}>
      {children}
    </MenuContext.Provider>
  )
}

export function useMenu() {
  const context = useContext(MenuContext)
  if (context === undefined) {
    throw new Error('useMenu must be used within a MenuProvider')
  }
  return context
} 