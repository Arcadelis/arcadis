// First, define an interface for the award structure
interface Award {
	name: string
	date: string
	// Add other properties that your award object has
}

// Assuming this is inside a component, define the awards prop or state
interface MonthlyAwardsProps {
	awards: Award[]
}

function MonthlyAwards({ awards }: MonthlyAwardsProps) {
	return (
		<>
			{awards.map((award: Award, index: number) => (
				<div key={`${award.name}-${award.date}`} className="flex flex-col items-center space-y-2">
					{/* ... rest of the award component code ... */}
				</div>
			))}
		</>
	)
}

export default MonthlyAwards
