// Genre types and options
export type Genre =
	| 'all-genres'
	| 'platformer'
	| 'adventure'
	| 'strategy'
	| 'fighting'
	| 'survival'
	| 'simulation'

export const genreOptions = [
	{ value: 'all-genres', label: 'All Genres' },
	{ value: 'platformer', label: 'Platformer' },
	{ value: 'adventure', label: 'Adventure' },
	{ value: 'strategy', label: 'Strategy' },
	{ value: 'fighting', label: 'Fighting' },
	{ value: 'survival', label: 'Survival' },
	{ value: 'simulation', label: 'Simulation' },
] as const

// Price types and options
export type Price = 'all-prices' | 'free' | 'under-15' | '15-25' | 'over-25'

export const priceOptions = [
	{ value: 'all-prices', label: 'All Prices' },
	{ value: 'free', label: 'Free' },
	{ value: 'under-15', label: 'Under $15' },
	{ value: '15-25', label: '$15 – $25' },
	{ value: 'over-25', label: 'Over $25' },
] as const

// Rating types and options
export type Rating = 'all-ratings' | '4-plus-stars' | '3-plus-stars' | '2-plus-stars'

export const ratingOptions = [
	{ value: 'all-ratings', label: 'All Ratings' },
	{ value: '4-plus-stars', label: '4+ Stars' },
	{ value: '3-plus-stars', label: '3+ Stars' },
	{ value: '2-plus-stars', label: '2+ Stars' },
] as const
