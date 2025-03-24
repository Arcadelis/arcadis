import { Globe, Shield } from 'lucide-react'
import React from 'react'

const GamingPlatform = () => {
	return (
		<div className="min-h-screen w-full bg-gradient-to-br from-[#1A2B3C] via-[#0A1525] to-[#070B1A] p-8">
			<div className="max-w-7xl mx-auto text-center">
				<h1 className="text-4xl font-bold text-white mb-4">
					Empowering Gamers and Game Developers Alike
				</h1>
				<p className="text-[#00C2FF] text-lg mb-4">Powered by Stellar</p>
				<p className="text-gray-300 max-w-3xl mx-auto mb-16 leading-relaxed">
					At StarkPlay Hub, we bridge the gap between players seeking immersive experiences and
					developers aiming to showcase their creations. Our platform is designed to cater to the
					needs of both communities, fostering a thriving ecosystem where everyone benefits.
				</p>

				<div className="grid grid-cols-1 md:grid-cols-3 gap-6">
					{/* For Gamers Card */}
					<div className="bg-gradient-to-b from-[#0A1931] to-[#0A1525] backdrop-blur rounded-lg p-8 text-left flex flex-col items-center ">
						<div className=" flex flex-col items-center">
							<div className="w-12 h-12 mb-6">
								<Shield className="w-full h-full text-[#00FF9D]" />
							</div>
							<h2 className="text-white text-xl font-semibold mb-4">For Gamers</h2>
						</div>

						<p className="text-gray-300 text-sm mb-6 text-center leading-relaxed">
							Explore a library of Arcadis-based games, Engage in quests, write reviews, and
							participate in community votes to earn tokens, NFTs, and build your reputation.
						</p>
						<ul className="space-y-3 mb-8 w-full">
							{['Discover New Games', 'Earn Rewards', 'Connect with the Community'].map((item) => (
								<li key={item} className="flex items-center text-gray-300">
									<span className="text-[#00FF9D] mr-2">✓</span>
									{item}
								</li>
							))}
						</ul>
						<button
							type="button"
							className="text-gray-300 flex items-center hover:text-white transition-colors"
						>
							Learn More
							<span className="ml-2">→</span>
						</button>
					</div>

					{/* Powered by Blockchain Card */}
					<div className="bg-gradient-to-b from-[#0B1933] to-[#0A1525] backdrop-blur rounded-lg p-8 text-left flex flex-col items-center ">
						x
						<div className=" flex flex-col items-center">
							<div className=" flex flex-col items-center">
								<div className="w-12 h-12 mb-6">
									<Shield className="w-full h-full text-[#00FF9D]" />
								</div>
								<h2 className="text-white text-xl font-semibold mb-4">Powered by Blockchain</h2>
							</div>
						</div>
						<p className="text-gray-300 text-sm mb-6 leading-relaxed text-center">
							Every action on Arcadis is secured by Starknet zk-rollup technology, ensuring fast and
							trustworthy interactions.
						</p>
						<ul className="space-y-3 mb-8">
							{[
								'Transparent player and game data',
								'Immutable records for reviews, votes, and rewards',
								'Instant, low-cost transactions for seamless participation',
							].map((item) => (
								<li key={item} className="flex items-center text-gray-300">
									<span className="text-[#4D7CFF] mr-2">✓</span>
									{item}
								</li>
							))}
						</ul>
						<button
							type="button"
							className="text-gray-300 flex items-center hover:text-white transition-colors"
						>
							Learn More
							<span className="ml-2">→</span>
						</button>
					</div>

					{/* For Game Developers Card */}
					<div className="bg-gradient-to-b from-[#0C1935] to-[#0A1525] backdrop-blur rounded-lg p-8 text-left flex flex-col items-center ">
						<div className="w-12 h-12 mb-6">
							<Globe className="w-full h-full text-[#B96BFF]" />
						</div>
						<h2 className="text-white text-xl font-semibold mb-4">For Game Developers</h2>
						<p className="text-gray-300 text-sm mb-6 leading-relaxed text-center">
							List your Arcadis-based game to gain visibility, Receive valuable insights from player
							reviews and ratings to refine your game
						</p>
						<ul className="space-y-3 mb-8 w-full">
							{['Showcase Your Game', 'Community Feedback', 'Crowdfunding Opportunities'].map(
								(item) => (
									<li key={item} className="flex items-center text-gray-300">
										<span className="text-[#B96BFF] mr-2">✓</span>
										{item}
									</li>
								),
							)}
						</ul>
						<button
							type="button"
							className="text-gray-300 flex items-center hover:text-white transition-colors"
						>
							Learn More
							<span className="ml-2">→</span>
						</button>
					</div>
				</div>
			</div>
		</div>
	)
}

export default GamingPlatform
