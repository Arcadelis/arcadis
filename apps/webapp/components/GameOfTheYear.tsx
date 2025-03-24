import React from 'react'

interface Game {
	title: string
	category: string
	year: string
}

const GameOfTheYear = () => {
	const games: Game[] = [
		{
			title: 'Crypto Legends',
			category: 'Epic Quest',
			year: '2024',
		},
		{
			title: 'StarkWorld',
			category: 'Virtual Reality',
			year: '2023',
		},
	]

	return (
		<div className="bg-[#111827] p-8 max-w-md">
			{/* Trophy Icon Circle */}
			<div className="w-16 h-16 bg-[#FDB913] rounded-full flex items-center justify-center mb-6">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					className="w-8 h-8 text-black"
					role="img"
					aria-labelledby="trophyTitle"
					aria-label="trophyTitle"
				>
					<path
						strokeLinecap="round"
						strokeLinejoin="round"
						strokeWidth={2}
						d="M8 21h8m-4-4v4m-8-17h16M8 4v8a4 4 0 004 4 4 4 0 004-4V4"
					/>
				</svg>
			</div>

			{/* Title Section */}
			<h2 className="text-white text-2xl font-bold mb-2">Game of the Year</h2>
			<p className="text-gray-400 text-lg mb-6">
				The most outstanding game that pushed boundaries and captivated players
			</p>

			{/* Game Cards */}
			<div className="space-y-4">
				{games.map((game) => (
					<div
						key={game.title}
						className="bg-[#1A1B23] p-4 rounded-lg cursor-pointer hover:bg-[#2A2B33] transition-colors"
					>
						<h3 className="text-white text-lg font-semibold mb-1">{game.title}</h3>
						<div className="text-gray-400 text-sm">
							{game.category} • {game.year}
						</div>
					</div>
				))}
			</div>
		</div>
	)
}

export default GameOfTheYear
