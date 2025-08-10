import Image from 'next/image'
import React, { ReactNode } from 'react'

interface StatsCardProp {
    title: string
    value: string
    subText: string
    icon: string
    color: string
}

export const mockStatsData: StatsCardProp[] = [
  {
    title: "Total Revenue",
    value: "$27,164.97",
    subText: "+12% from last month",
    icon: "/svgs/revenue.svg", 
    color: "#00FF88" 
  },
  {
    title: "Downloads",
    value: "2,103",
    subText: "+8% from last month",
    icon: "/svgs/downloads.svg", 
    color: "#00D4FF" 
  },
  {
    title: "Published",
    value: "2",
    subText: "1 in draft",
    icon: "/svgs/published.svg", 
    color: "#A855F7" 
  },
  {
    title: "Avg Rating",
    value: "4.7",
    subText: "Across all games",
    icon: "/svgs/rating.svg", 
    color: "#FACC15" 
  }
];

function StatsCard({title, value, subText, icon, color}: StatsCardProp) {
  return (
    <div 
      className="w-[14.6rem] h-[8.7rem] bg-[#121417] p-[1.5rem] rounded-[0.5rem] border-2 flex flex-col"
      style={{ borderColor: color }}
    >
        <div className='w-full flex justify-between items-center'>
            <p className='text-sm font-bold text-[#9CA3AF] uppercase'>{title}</p>
            <Image src={icon} alt={title} width={24} height={24} />
        </div>
        <p className='text-[1.875rem] font-black' style={{color: color}}>{value}</p>
        
        <div className='flex items-center text-xs text-[#9CA3AF]'>{(title === "Total Revenue" || title === "Downloads") && (<Image src="/svgs/trend.svg" alt={title} width={14} height={12} />)}{subText}</div>
    </div>
  )
}

export default StatsCard