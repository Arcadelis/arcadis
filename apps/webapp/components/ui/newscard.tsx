import Image from 'next/image'

interface CardProps {
	img: string
	alt: string
	aspectRatio?: string
	heading: string
	description: string
	category: string
	date?: string
	categoryColor?: string
	cardBorderRadius?: string
	backgroundColor?: string
}

const NewsCard = ({
	img,
	alt,
	heading,
	description,
	category,
	categoryColor = 'green',
	cardBorderRadius = 'xl',
}: CardProps) => {
	return (
		<div
			className={`rounded-${cardBorderRadius} backdrop-blur-sm bg-white/5 overflow-hidden h-full shadow-lg hover:shadow-purple-500/20 transition-all duration-300 border border-white/10 p-4`}
		>
			<div className="relative h-52 overflow-hidden">
				<Image
					className="object-cover hover:scale-105 transition-transform duration-500 rounded-lg"
					src={img}
					alt={alt}
					fill
					sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
				/>
			</div>
			<div className="p-5 space-y-4 opacity-80">
				<div className="text-white">
					<h2 className="font-bold text-xl leading-tight">{heading}</h2>
				</div>
				<div className="text-gray-300">
					<p className="line-clamp-3 text-sm">{description}</p>
				</div>
				<div className={`text-${categoryColor}-300 font-medium`}>
					<span
						className={`inline-block px-2 py-1 bg-${categoryColor}-700 rounded-lg text-xs font-medium`}
					>
						{category}
					</span>
				</div>
			</div>
		</div>
	)
}

export default NewsCard
