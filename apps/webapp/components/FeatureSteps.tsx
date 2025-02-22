import type React from 'react'

interface Step {
	stepNumber: number
	title: string
	description: string
}

interface FeatureStepsProps {
	steps: Step[]
	ctaText: string
	onCtaClick: () => void
}

const FeatureSteps: React.FC<FeatureStepsProps> = ({ steps, ctaText, onCtaClick }) => {
	return (
		<section className="w-full">
			<div className="max-w-6xl mx-auto text-center">
				{/* Steps Container */}
				<div className="flex items-center justify-between md:justify-start  lg:justify-between flex-wrap lg:flex-nowrap gap-[12px]">
					{steps.map((step) => (
						<div
							key={step.stepNumber}
							className="w-full md:w-[30%]   lg:w-[20%] bg-[#F3F4F6] rounded-[8px] h-[250.5px] p-[25px]"
						>
							<div className="w-full flex items-center justify-start gap-[12px]">
								<div className="w-[32px] h-[32px] bg-[#EFF6FF] rounded-full flex items-center justify-center">
									<p className="text-[#9333EA] m-0 p-0 font-semibold text-[14px]">
										{step.stepNumber}
									</p>
								</div>
								<p className="text-[#111827] text-left font-semibold m-0 p-0 text-[17px] md:text-[15px]">
									{step.title}
								</p>
							</div>
							<div className="w-full mt-[16px]">
								<p className="text-[#4B5563] text-left font-normal text-[14px] md:text-[13.56px]">
									{step.description}
								</p>
							</div>
						</div>
					))}
				</div>
				<button
					type="button"
					className="mt-[48px] px-[32px] w-[195px] py-[10px] bg-[#06B6D4] text-white text-[13px] font-semibold rounded-[6px] hover:bg-[#2ab5ce] transition"
					onClick={onCtaClick}
				>
					{ctaText}
				</button>
			</div>
		</section>
	)
}

export default FeatureSteps
