'use client'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { motion } from 'framer-motion'
import { ArrowRight, DiscIcon as Discord, Github, Twitter } from 'lucide-react'
import Image from 'next/image'
import { useState } from 'react'

import FeatureSteps from '@/components/FeatureSteps'
import SearchGamesBar from '@/components/SearchGamesBar'

export default function Home() {
	const [email, setEmail] = useState('')

	// Calculate time until launch (example: March 1st, 2025)
	const launchDate = new Date('2025-03-01').getTime()
	const now = new Date().getTime()
	const distance = launchDate - now
	const days = Math.floor(distance / (1000 * 60 * 60 * 24))
	const stepsData = [
		{
			stepNumber: 1,
			title: 'Game Visibility',
			description: 'Showcase your game to a passionate and growingStellar community.',
		},
		{
			stepNumber: 2,
			title: 'Community Insights',
			description:
				'Leverage real-time reviews,ratings, and user feedback to refine your game and boost player satisfaction.',
		},
		{
			stepNumber: 3,
			title: 'Reputation Building',
			description:
				'Utilize Arcadis features like community votes, tutorials, and achievements to establish your game as a trusted and popular title.',
		},
		{
			stepNumber: 4,
			title: 'Crowdfunding',
			description:
				'Access crowdfunding tools like new integration to fund game updates or new features while gaining community support.',
		},
		{
			stepNumber: 5,
			title: 'Growth and Promotion',
			description:
				'Increase player acquisition, retention, and in-game engagement while leveraging the platform to promote your game effectively.',
		},
	]
	const handleCtaClick = () => {
		console.log('CTA Clicked!')
	}

	return (
		<main className="min-h-screen relative overflow-hidden bg-[#070B1D]">
			{/* Background gradient */}
			<div className="absolute inset-0 bg-gradient-to-b from-transparent via-purple-500/5 to-transparent" />

			{/* Floating elements */}
			<div className="absolute inset-0 overflow-hidden">
				{[...Array(3)].map((_, i) => (
					<motion.div
						// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
						key={i}
						className="absolute w-[300px] h-[300px] rounded-full bg-blue-500/10"
						animate={{
							x: [0, 100, 0],
							y: [0, 50, 0],
							scale: [1, 1.1, 1],
						}}
						transition={{
							duration: 15,
							repeat: Number.POSITIVE_INFINITY,
							delay: i * 2,
						}}
						style={{
							left: `${Math.random() * 100}%`,
							top: `${Math.random() * 100}%`,
						}}
					/>
				))}
			</div>

			<div className="relative z-10 container mx-auto px-4 py-20 flex flex-col items-center justify-center min-h-screen gap-12">
				{/* Main content */}
				<motion.div
					initial={{ opacity: 0, y: 20 }}
					animate={{ opacity: 1, y: 0 }}
					transition={{ duration: 0.8 }}
					className="max-w-4xl mx-auto text-center space-y-8"
				>
					<div className="inline-block px-4 py-1.5 bg-gradient-to-r from-blue-500/10 to-purple-500/10 rounded-full backdrop-blur-sm border border-blue-500/20 text-blue-300 mb-4">
						{days} days until launch
					</div>

					<h1 className="text-5xl md:text-7xl font-bold tracking-tight text-white mb-8">
						Coming Soon{' '}
						<span className="bg-gradient-to-r from-[#3B82F6] via-[#60A5FA] to-[#93C5FD] text-transparent bg-clip-text">
							ARCADIS Gaming
						</span>
					</h1>

					<p className="text-xl md:text-2xl text-gray-400 max-w-2xl mx-auto">
						Get ready for the next evolution in blockchain gaming. Join thousands of players in
						shaping the future of gaming.
					</p>

					{/* Newsletter signup */}
					<div className="max-w-md mx-auto mt-12">
						<div className="flex gap-2">
							<Input
								type="email"
								placeholder="Enter your email"
								className="bg-white/5 border-white/10"
								value={email}
								onChange={(e) => setEmail(e.target.value)}
							/>
							<Button className="bg-blue-600 hover:bg-blue-700">
								Notify Me <ArrowRight className="ml-2 h-4 w-4" />
							</Button>
						</div>
						<p className="text-sm text-gray-500 mt-2">Be the first to know when we launch</p>
					</div>
				</motion.div>

				{/* Stats */}
				<motion.div
					initial={{ opacity: 0, y: 20 }}
					animate={{ opacity: 1, y: 0 }}
					transition={{ duration: 0.8, delay: 0.2 }}
					className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl w-full mt-12"
				>
					{[
						{ label: 'Pre-registered Players', value: '10,000+' },
						{ label: 'Games at Launch', value: '50+' },
						{ label: 'Total Prize Pool', value: '$1M+' },
					].map((stat, i) => (
						<div
							// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
							key={i}
							className="text-center p-6 rounded-xl bg-white/5 backdrop-blur-sm border border-white/10"
						>
							<div className="text-3xl font-bold text-white mb-2">{stat.value}</div>
							<div className="text-gray-400">{stat.label}</div>
						</div>
					))}
				</motion.div>
				<SearchGamesBar />
				<FeatureSteps steps={stepsData} ctaText="Register Your Game" onCtaClick={handleCtaClick} />

				{/* Stats */}
				<motion.div
					initial={{ opacity: 0, y: 20 }}
					animate={{ opacity: 1, y: 0 }}
					transition={{ duration: 0.8, delay: 0.2 }}
					className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-7xl  w-full mt-12"
				>
					{[
						{
							icon: '/assets/images/teal-icon.svg',
							headline: 'Curated Games',
							description:
								'Discover and explore handpicked Stellar games with in-depth reviews, ratings, and personalized recommendations tailored for you.',
						},
						{
							icon: '/assets/images/purple-icon.svg',
							headline: 'Community Rewards',
							description:
								' Earn tokens, NFTs, and reputation for playing, reviewing, voting, and contributing.',
						},
						{
							icon: '/assets/images/pink-icon.svg',
							headline: 'Support Innovation',
							description: 'Help developers crowdfund and grow their games with your engagement.',
						},
					].map((feature, i) => (
						<div
							// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
							key={i}
							className=" p-6 rounded-xl backdrop-blur-lg relative
							bg-[#3c07646A] 
							bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)]
							text-white w-auto"
						>
							<div className="w-full flex items-center gap-x-4">
								<Image
									src={feature.icon || ''}
									alt={feature.headline}
									width={48}
									height={48}
									quality={90}
								/>
								<p className="text-[1.75rem] font-bold">{feature.headline}</p>
							</div>
							<p className="font-normal text-[0.95rem] mt-[1rem] text-left">
								{feature.description}
							</p>
						</div>
					))}
				</motion.div>
				{/* Social links */}
				<motion.div
					initial={{ opacity: 0 }}
					animate={{ opacity: 1 }}
					transition={{ duration: 0.8, delay: 0.4 }}
					className="flex gap-6 mt-12"
				>
					{[
						{ icon: Discord, label: 'Discord' },
						{ icon: Twitter, label: 'Twitter' },
						{ icon: Github, label: 'Github' },
					].map((social, i) => (
						<Button
							// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
							key={i}
							variant="ghost"
							size="icon"
							className="text-gray-400 hover:text-white hover:bg-white/10"
						>
							<social.icon className="h-5 w-5" />
							<span className="sr-only">{social.label}</span>
						</Button>
					))}
				</motion.div>
			</div>
		</main>
	)
}
