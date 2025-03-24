import {
	BarChart3,
	ChevronDown,
	Gamepad2,
	Gift,
	Globe,
	LayoutGrid,
	Lightbulb,
	MonitorPlay,
	MoveHorizontal,
	Network,
	Search,
	Users,
	Wallet,
} from 'lucide-react'
// pages/index.tsx
import type { NextPage } from 'next'
import Head from 'next/head'

import { Menu, Transition } from '@headlessui/react'

const HomeHeroPage: NextPage = () => {
	const categoryOptions = ['Action', 'Adventure', 'Puzzle', 'Strategy']
	const stateOptions = ['Active', 'Inactive', 'Upcoming']
	const frameworkOptions = ['Unity', 'Unreal Engine', 'Godot']

	return (
		<div className="min-h-screen bg-[#080B27] text-white">
			<Head>
				<title>Stellar Gaming Platform | Discover. Play. Earn.</title>
				<meta name="description" content="Shape the future of blockchain gaming" />
				<link rel="icon" href="/favicon.ico" />
			</Head>

			<main className="container mx-auto px-4 py-12 max-w-7xl">
				{/* Hero Section */}
				<section className="flex flex-col items-center justify-center text-center mt-8 mb-20 relative">
					{/* Decorative dots */}
					<div className="absolute top-0 left-1/4 w-2 h-2 rounded-full bg-[#060A32] opacity-60" />
					<div className="absolute bottom-1/3 right-1/4 w-2 h-2 rounded-full bg-blue-300 opacity-60" />

					<h1 className="text-5xl md:text-6xl font-bold mb-6">Discover. Play. Earn.</h1>
					<h2 className="text-4xl md:text-5xl font-bold mb-12">
						<span className="bg-gradient-to-r from-blue-500 to-cyan-400 bg-clip-text text-transparent">
							Shape the Future of Blockchain
						</span>
						<br />
						<span className="bg-gradient-to-r from-cyan-400 to-blue-300 bg-clip-text text-transparent">
							Gaming
						</span>
					</h2>

					<div className="flex flex-col sm:flex-row gap-4 z-10">
						<button
							type="button"
							className="bg-gradient-to-r from-[#3dadd6] to-[#164E63] text-white py-3 px-8 rounded-lg transition-all duration-300"
						>
							Explore Games Now
						</button>
						<button
							type="button"
							className="border bg-[#060A32] border-gray-600 hover:border-gray-400 text-white py-3 px-8 rounded-lg transition-all duration-300"
						>
							Register Your Game
						</button>
					</div>
				</section>

				{/* Features Section */}
				<section className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-20">
					<div
						className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl"
					>
						<div className="bg-teal-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
							<Gamepad2 className="text-white" size={24} />
						</div>
						<h3 className="text-xl font-bold mb-2">Curated Games</h3>
						<p className="text-gray-300">
							Discover and explore handpicked Stellar games with in-depth reviews, ratings, and
							personalized recommendations tailored for you.
						</p>
					</div>

					<div
						className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl"
					>
						<div className="bg-purple-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
							<Users className="text-white" size={24} />
						</div>
						<h3 className="text-xl font-bold mb-2">Community Rewards</h3>
						<p className="text-gray-300">
							Earn tokens, NFTs, and reputation for playing, reviewing, voting, and contributing.
						</p>
					</div>

					<div
						className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl"
					>
						<div className="bg-pink-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
							<Lightbulb className="text-white" size={24} />
						</div>
						<h3 className="text-xl font-bold mb-2">Support Innovation</h3>
						<p className="text-gray-300">
							Help developers crowdfund and grow their games with your engagement.
						</p>
					</div>
				</section>

				<section className="mb-20 px-4 sm:px-6 lg:px-8">
					<div className="bg-white md:rounded-full rounded p-2 flex flex-row items-center gap-2 sm:gap-4 ">
						{/* Search Input */}
						<input
							type="text"
							placeholder="Search Games"
							className="bg-transparent border-none outline-none pl-4 flex-grow text-black w-[50%] h-full sm:w-auto md:rounded-full transition-all"
							aria-label="Search games"
						/>

						{/* Buttons and Filters */}
						<div className="flex flex-wrap items-center gap-2 sm:gap-4">
							{/* Mobile Dropdown (Visible only on small screens) */}
							<div className="sm:hidden">
								<Menu as="div" className="relative">
									<Menu.Button className="flex items-center text-black px-3 py-2 hover:bg-gray-100 rounded-full transition-all">
										<span>Filters</span>
										<ChevronDown size={16} className="ml-2" />
									</Menu.Button>
									<Transition
										enter="transition ease-out duration-100"
										enterFrom="transform opacity-0 scale-95"
										enterTo="transform opacity-100 scale-100"
										leave="transition ease-in duration-75"
										leaveFrom="transform opacity-100 scale-100"
										leaveTo="transform opacity-0 scale-95"
									>
										<Menu.Items className="absolute right-0 mt-2 w-48 origin-top-right bg-white rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-10">
											{/* Category Dropdown */}
											<Menu.Item>
												{({ active }) => (
													<div className={`${active ? 'bg-gray-100' : ''} p-2`}>
														<span className="block text-sm text-gray-700 font-medium px-4 py-2">
															Category
														</span>
														{categoryOptions.map((option) => (
															<button
																type="button"
																key={option}
																className="block w-full px-4 py-2 text-sm text-gray-700 text-left hover:bg-gray-100"
															>
																{option}
															</button>
														))}
													</div>
												)}
											</Menu.Item>

											{/* State Dropdown */}
											<Menu.Item>
												{({ active }) => (
													<div className={`${active ? 'bg-gray-100' : ''} p-2`}>
														<span className="block text-sm text-gray-700 font-medium px-4 py-2">
															State
														</span>
														{stateOptions.map((option) => (
															<button
																type="button"
																key={option}
																className="block w-full px-4 py-2 text-sm text-gray-700 text-left hover:bg-gray-100"
															>
																{option}
															</button>
														))}
													</div>
												)}
											</Menu.Item>

											{/* Framework Dropdown */}
											<Menu.Item>
												{({ active }) => (
													<div className={`${active ? 'bg-gray-100' : ''} p-2`}>
														<span className="block text-sm text-gray-700 font-medium px-4 py-2">
															Framework
														</span>
														{frameworkOptions.map((option) => (
															<button
																type="button"
																key={option}
																className="block w-full px-4 py-2 text-sm text-gray-700 text-left hover:bg-gray-100"
															>
																{option}
															</button>
														))}
													</div>
												)}
											</Menu.Item>
										</Menu.Items>
									</Transition>
								</Menu>
							</div>

							{/* Desktop Buttons (Hidden on small screens) */}
							<div className="hidden sm:flex flex-wrap items-center gap-2 sm:gap-4">
								{/* Category Dropdown */}
								<div className="relative">
									<Menu as="div" className="relative">
										<Menu.Button className="flex items-center text-black px-3 py-2 hover:bg-gray-100 rounded-full transition-all">
											<span>Choose category</span>
											<ChevronDown size={16} className="ml-2" />
										</Menu.Button>
										<Transition
											enter="transition ease-out duration-100"
											enterFrom="transform opacity-0 scale-95"
											enterTo="transform opacity-100 scale-100"
											leave="transition ease-in duration-75"
											leaveFrom="transform opacity-100 scale-100"
											leaveTo="transform opacity-0 scale-95"
										>
											<Menu.Items className="absolute right-0 mt-2 w-48 origin-top-right bg-white rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-10">
												{categoryOptions.map((option) => (
													<Menu.Item key={option}>
														{({ active }) => (
															<button
																type="button"
																className={`${
																	active ? 'bg-gray-100' : ''
																} block w-full px-4 py-2 text-sm text-gray-700 text-left`}
															>
																{option}
															</button>
														)}
													</Menu.Item>
												))}
											</Menu.Items>
										</Transition>
									</Menu>
								</div>

								{/* Divider */}
								<div className="h-8 w-px bg-gray-300" />

								{/* State Dropdown */}
								<div className="relative">
									<Menu as="div" className="relative">
										<Menu.Button className="flex items-center text-black px-3 py-2 hover:bg-gray-100 rounded-full transition-all">
											<span>Choose state</span>
											<ChevronDown size={16} className="ml-2" />
										</Menu.Button>
										<Transition
											enter="transition ease-out duration-100"
											enterFrom="transform opacity-0 scale-95"
											enterTo="transform opacity-100 scale-100"
											leave="transition ease-in duration-75"
											leaveFrom="transform opacity-100 scale-100"
											leaveTo="transform opacity-0 scale-95"
										>
											<Menu.Items className="absolute right-0 mt-2 w-48 origin-top-right bg-white rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-10">
												{stateOptions.map((option) => (
													<Menu.Item key={option}>
														{({ active }) => (
															<button
																type="button"
																className={`${
																	active ? 'bg-gray-100' : ''
																} block w-full px-4 py-2 text-sm text-gray-700 text-left`}
															>
																{option}
															</button>
														)}
													</Menu.Item>
												))}
											</Menu.Items>
										</Transition>
									</Menu>
								</div>

								{/* Divider */}
								<div className="h-8 w-px bg-gray-300" />

								{/* Framework Dropdown */}
								<div className="relative">
									<Menu as="div" className="relative">
										<Menu.Button className="flex items-center text-black px-3 py-2 hover:bg-gray-100 rounded-full transition-all">
											<span>Framework</span>
											<ChevronDown size={16} className="ml-2" />
										</Menu.Button>
										<Transition
											enter="transition ease-out duration-100"
											enterFrom="transform opacity-0 scale-95"
											enterTo="transform opacity-100 scale-100"
											leave="transition ease-in duration-75"
											leaveFrom="transform opacity-100 scale-100"
											leaveTo="transform opacity-0 scale-95"
										>
											<Menu.Items className="absolute right-0 mt-2 w-48 origin-top-right bg-white rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-10">
												{frameworkOptions.map((option) => (
													<Menu.Item key={option}>
														{({ active }) => (
															<button
																type="button"
																className={`${
																	active ? 'bg-gray-100' : ''
																} block w-full px-4 py-2 text-sm text-gray-700 text-left`}
															>
																{option}
															</button>
														)}
													</Menu.Item>
												))}
											</Menu.Items>
										</Transition>
									</Menu>
								</div>
							</div>
						</div>
						{/* Search Button */}
						<button
							type="button"
							className="bg-blue-500 hover:bg-blue-600 p-3 rounded-full text-white transition-all focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
							aria-label="Search"
						>
							<Search size={20} />
						</button>
					</div>
				</section>

				{/* Categories Section */}
				<section className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-6 text-center">
					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<Gift className="text-white" size={20} />
						</div>
						<span className="text-sm">Play-to-Earn</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<Globe className="text-white" size={20} />
						</div>
						<span className="text-sm">Gaming Metaverses</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<LayoutGrid className="text-white" size={20} />
						</div>
						<span className="text-sm">DeFi Gaming</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<Wallet className="text-white" size={20} />
						</div>
						<span className="text-sm">Trading Card Games</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<Network className="text-white" size={20} />
						</div>
						<span className="text-sm">Blockchain RPGs</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<MoveHorizontal className="text-white" size={20} />
						</div>
						<span className="text-sm">Move-to-Earn</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<BarChart3 className="text-white" size={20} />
						</div>
						<span className="text-sm">Strategy Games</span>
					</div>

					<div className="flex flex-col items-center space-y-2">
						<div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
							<MonitorPlay className="text-white" size={20} />
						</div>
						<span className="text-sm">Simulators</span>
					</div>
				</section>
			</main>
		</div>
	)
}

export default HomeHeroPage
