import { ChevronDown, Search } from 'lucide-react'
import Image from 'next/image'

const SearchGamesBar = () => {
	const categories = [
		{ title: 'Play-to-Earn', icon: '/assets/images/play-to-earn.svg' },
		{ title: 'Gaming Metaverses', icon: '/assets/images/globe.svg' },
		{ title: 'DeFi Gaming', icon: '/assets/images/defi-gaming.svg' },
		{ title: 'Trading Card Games', icon: '/assets/images/trading-cards.svg' },
		{ title: 'Blockchain RPGs', icon: '/assets/images/rpgs.svg' },
		{ title: 'Move-to-Earn', icon: '/assets/images/move-to-earn.svg' },
		{ title: 'Strategy Games', icon: '/assets/images/strategy-games.svg' },
		{ title: 'Simulators', icon: '/assets/images/simulators.svg' },
	]

	return (
		<div className="w-full mt-6 px-4">
			<div className="max-w-6xl mx-auto">
				<div className="w-full h-14 rounded-full px-4 bg-white flex items-center justify-between shadow-md">
					<div className="flex flex-1 gap-4 items-center ">
						<p className="font-semibold text-sm text-purple-500">Search Games</p>

						<div className="hidden md:flex gap-6">
							{['Choose category', 'Choose state', 'Framework'].map((item) => (
								<div key={item} className="flex items-center gap-6 cursor-pointer">
									<p className="text-sm text-[#0A0A0A] font-normal">{item}</p>
									<ChevronDown color="#0A0A0A" size={14} />
								</div>
							))}
						</div>
					</div>

					<div className="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center cursor-pointer">
						<Search color="white" size={16} />
					</div>
				</div>

				<div className="w-full mt-12">
					<div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-8 gap-4 place-items-center">
						{categories.map(({ icon, title }) => (
							<div key={title} className="flex flex-col items-center w-auto justify-center">
								<Image src={icon} width={24} height={24} alt={title} className="w-6 h-6" />
								<p className="text-white font-medium mt-2 text-[13.53px] text-center">{title}</p>
							</div>
						))}
					</div>
				</div>
			</div>
		</div>
	)
}

export default SearchGamesBar
