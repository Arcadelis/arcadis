"use client"

import { SlidersHorizontal, Search } from "lucide-react"
import { FilterDropdown } from "./FilterDropdown"
import { type Genre, genreOptions, type Price, priceOptions, type Rating, ratingOptions } from "@/lib/games/filters"

interface Props {
  value: string
  onChange: (v: string) => void
  genre: Genre
  price: Price
  rating: Rating
  onFiltersChange: (f: { genre: Genre; price: Price; rating: Rating }) => void
}

export function SearchBar({ value, onChange, genre, price, rating, onFiltersChange }: Props) {
  return (
    <div className="rounded-[8px] border border-[#16213E] bg-[#121417] p-3 sm:p-4 md:p-[25px]">
      <div className="flex flex-col gap-4 md:flex-row md:items-center md:gap-4">
        {/* Input */}
        <div className="relative w-full md:flex-1">
          <Search className="pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 h-5 w-5 text-[#00D4FF]" />
          <input
            type="text"
            value={value}
            onChange={(e) => onChange(e.target.value)}
            placeholder="Search for your next adventure..."
            className="h-12 w-full rounded-[6px] border border-[#16213E] bg-[#050508]
                       pl-12 pr-3 text-[14px] leading-[18px] text-white placeholder:text-[#6B7280]
                       focus:outline-none focus:ring-2 focus:ring-[#00D4FF]"
          />
        </div>

        {/* Dropdowns */}
        <div
          className="
            flex w-full flex-wrap gap-3 md:gap-4
            md:w-auto md:flex-nowrap
          "
        >
          <div className="w-full sm:w-[calc(50%-0.375rem)] lg:w-auto">
            <FilterDropdown
              label="Genre"
              icon={<SlidersHorizontal className="h-4 w-4 text-[#00D4FF]" />}
              value={genre}
              options={[...genreOptions]}
              onChange={(v) => onFiltersChange({ genre: v as Genre, price, rating })}
              ariaLabel="Filter by genre"
            />
          </div>

          <div className="w-full sm:w-[calc(50%-0.375rem)] lg:w-auto">
            <FilterDropdown
              label="Price"
              value={price}
              options={[...priceOptions]}
              onChange={(v) => onFiltersChange({ genre, price: v as Price, rating })}
              ariaLabel="Filter by price"
            />
          </div>

          <div className="w-full sm:w-[calc(50%-0.375rem)] lg:w-auto">
            <FilterDropdown
              label="Rating"
              value={rating}
              options={[...ratingOptions]}
              onChange={(v) => onFiltersChange({ genre, price, rating: v as Rating })}
              ariaLabel="Filter by rating"
            />
          </div>
        </div>
      </div>
    </div>
  )
}
