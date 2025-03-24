'use client'
import NewsCard from '@/components/ui/newscard'
import { motion } from 'framer-motion'
import data from './data'

const News = () => {
	return (
		<main className="min-h-screen relative bg-[#070B1D] p-8">
			<div className="absolute inset-0 bg-gradient-to-b from-transparent via-purple-500/5 to-transparent" />
			<motion.div className="max-w-7xl mx-auto">
				<h1 className="text-white font-semibold text-3xl mb-8 pl-2">News</h1>
				<div>
					{data && data.length > 0 ? (
						<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
							{data.map((dataItem) => (
								<div key={dataItem.headline} className="h-full">
									<NewsCard
										img={dataItem.imageUrl}
										alt={dataItem.imageAlt}
										heading={dataItem.headline}
										description={dataItem.description}
										category={dataItem.category}
									/>
								</div>
							))}
						</div>
					) : (
						<div className="text-white text-center p-8">Data not found</div>
					)}
				</div>
			</motion.div>
		</main>
	)
}
export default News
