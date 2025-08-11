import Image from 'next/image'
import Link from 'next/link'
import React from 'react'

function Navbar() {
  return (
    <nav className='w-full bg-[#050508CC] h-[5.3rem] fixed top-0 left-0 px-[3.625rem] flex justify-between items-center z-50'>
        <Image src="/svgs/logo.svg" alt='arcadis-marketplace' width={187} height={52} quality={90} />

        <div className='flex items-center gap-x-[1.5rem]'>
            <Link href="/" className='text-base text-[#94A3B8]'>Browse Games</Link>
            <Link href="/" className='text-base text-[#94A3B8]'>Developer Hub</Link>
            <button className='w-[4.9375rem] h-[2.5rem] rounded-[0.375rem] bg-[#0D0F11] border border-[#16213E] text-sm font-medium cursor-pointer'>Sign In</button>
            <button className='w-[5.1875rem] h-[2.5rem] rounded-[0.375rem] bg-gradient-to-br from-[#00D4FF] to-[#A855F7] text-sm text-[#0D0F11] font-medium cursor-pointer'>sign Up</button>
        </div>
    </nav>
  )
}

export default Navbar