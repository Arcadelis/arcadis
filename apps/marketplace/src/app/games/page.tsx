"use client"

import { useState } from "react"
import { SearchBar } from "@/components/games/SearchBar"
import type { Genre, Price, Rating } from "@/lib/games/filters"

export default function GamesPage() {
  const [searchValue, setSearchValue] = useState("")
  const [genre, setGenre] = useState<Genre>("all-genres")
  const [price, setPrice] = useState<Price>("all-prices")
  const [rating, setRating] = useState<Rating>("all-ratings")

  return (
    <div className="min-h-screen bg-[#0B0C10]">
      {/* Hero */}
      <section className="mx-auto max-w-6xl px-4 pb-6 pt-8 text-center">
        <h1
          className="text-3xl md:text-5xl font-extrabold tracking-tight
                     bg-gradient-to-r from-[#00D4FF] to-[#A855F7] bg-clip-text text-transparent"
        >
          GAME LIBRARY
        </h1>
        <p className="mt-3 text-sm md:text-base text-[#9CA3AF]">
          Discover your next gaming obsession
        </p>
      </section>

      {/* Search + Filters bar */}
      <section className="mx-auto max-w-5xl">
        <SearchBar
          value={searchValue}
          onChange={setSearchValue}
          genre={genre}
          price={price}
          rating={rating}
          onFiltersChange={({ genre, price, rating }) => {
            setGenre(genre)
            setPrice(price)
            setRating(rating)
          }}
        />
      </section>
    </div>
  )
}
