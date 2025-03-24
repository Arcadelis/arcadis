import Image from 'next/image'

interface MonthlyAwardsProps {
	month: string
	year: string
	icon: string
	backgroundColor: string
	awards: MonthlyAwardCardProps[]
	maxCategories: number
	showIcons: boolean
	cardShape: string //in px to specify roundness
}
export default function MonthlyAwards({
	icon,
	month,
	year,
	backgroundColor,
	awards,
	showIcons,
	cardShape,
}: MonthlyAwardsProps) {
	return (
		<div className={`w-[624px] h-[456px] bg-[${backgroundColor}] p-5`}>
			<div className="flex space-x-3">
				<img src={icon} alt="" className="block" />
				<p className="text-[23px] text-white">
					{month} {year}
				</p>
			</div>
			<div className="flex flex-col space-y-3 mt-5">
				{awards?.map((award) => (
					<MonthlyAwardCard
						key={`${award.categoryName}-${award.winnerName}`}
						{...award}
						showIcon={showIcons}
						cardShape={cardShape}
					/>
				))}
			</div>
		</div>
	)
}
interface MonthlyAwardCardProps {
	id: string
	categoryIcon: string
	iconBackgroundColor: string
	categoryName: string
	winnerName: string
	winnerDetails: string
	showIcon: boolean
	cardShape: string
}

const MonthlyAwardCard = ({
	categoryIcon,
	showIcon,
	iconBackgroundColor,
	categoryName,
	winnerName,
	winnerDetails,
	cardShape,
}: MonthlyAwardCardProps) => {
	return (
		<div className="flex flex-col space-y-3">
			<div>{showIcon && <Image src={categoryIcon} alt="image" width={38} height={38} />}</div>
			<div
				className={`w-[567px] h-[100px] rounded-[${cardShape}] p-3 flex flex-col space-y-2 bg-[${iconBackgroundColor}]`}
			>
				<p className="text-[13.5px] text-[#9CA3AF]">{categoryName}</p>
				<p className="text-[15.5px] text-white">{winnerName}</p>
				<p className="text-[13.5px] text-[#9CA3AF]">{winnerDetails}</p>
			</div>
		</div>
	)
}
