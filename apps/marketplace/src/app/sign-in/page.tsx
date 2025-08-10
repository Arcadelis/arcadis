"use client";
import Image from "next/image";
import Link from "next/link";
import React, { useState } from "react";

function SignIn() {
  const [isPasswordVisible, setIsPasswordVisible] = useState<boolean>(false);
  return (
    <main className="w-full min-h-screen flex justify-center py-[5.5rem] bg-gradient-to-r from-[#00D4FF1A] to-[#A855F71A]">
      <div className="w-[28rem] flex flex-col items-center">
        <div className="flex items-center gap-2">
          <Image
            src="/svgs/logo-icon.svg"
            alt="arcadis_icon"
            width={40}
            height={28}
            quality={90}
            className="w-[4.5rem] h-[3.75rem]"
          />
          <div>
            <p className="bg-gradient-to-r from-[#00D4FF] to-[#A855F7] text-transparent bg-clip-text text-[1.875rem] font-bold">
              ARCADIS
            </p>
            <p className="text-[#94A3B8] text-xs text-center">MARKETPLACE</p>
          </div>
        </div>

        <p className="mt-[1.5rem] bg-gradient-to-r from-[#00D4FF] to-[#FFFFFF] text-transparent bg-clip-text text-[2.25rem] font-black">
          WELCOME BACK
        </p>
        <p className="text-[1.125rem] text-[#9CA3AF]">
          Sign in to your gaming account
        </p>

        <div className="w-[28rem] h-[37.75rem] rounded-[0.5rem] bg-[#121417] border-2 border-[#16213E] mt-[2rem] p-[1.625rem]">
          <p className="font-bold text-white text-[1.5rem] text-center mb-[1.5rem]">
            SIGN IN
          </p>

          <div className="w-full h-[3rem] bg-[#0D0F11] rounded-[0.375rem] border-2 border-[#16213E] flex justify-center items-start pt-2.5 text-sm cursor-pointer">
            <Image
              src="/svgs/chrome.svg"
              alt="google_chrome"
              width={16}
              height={16}
              quality={90}
              className="mr-[0.9rem]"
            />
            <p className="mb-1.5">Continue with Google</p>
          </div>

          <div className="w-full h-[3rem] bg-[#0D0F11] rounded-[0.375rem] border-2 border-[#16213E] flex justify-center items-start pt-2.5 text-sm mt-[0.75rem] cursor-pointer">
            <Image
              src="/svgs/github.svg"
              alt="github"
              width={16}
              height={16}
              quality={90}
              className="mr-[0.9rem]"
            />
            <p className="mb-1.5">Continue with Github</p>
          </div>

          <div className=" my-[2rem] relative w-full">
            <div className="w-[11.625rem] h-[1rem] absolute left-[6.675rem] -top-[0.5rem] bg-[#16213E]">
              <p className="uppercase text-[#9CA3AF] text-xs font-semibold text-center">
                Or continue with email
              </p>
            </div>
            <div className="w-full h-0.5 bg-[#16213E]" />
          </div>

          <form className="w-full">
            <div className="w-full">
              <label
                htmlFor="email"
                className="text-[#D1D5DB] text-sm font-semibold "
              >
                Email Address
              </label>
              <div className="w-full relative mt-[0.78rem]">
                <input
                  type="email"
                  name="email"
                  id="email"
                  placeholder="gamer@example.com"
                  className="w-full h-[3rem] outline-none border rounded-[0.375rem] border-[#16213E] bg-[#050508] text-white placeholder:text-[#6B7280] text-sm pl-12"
                />
                <Image
                  src="/svgs/email.svg"
                  alt="email"
                  width={20}
                  height={20}
                  className="absolute top-[1rem] left-[1rem]"
                />
              </div>
            </div>

            <div className="w-full mt-[1rem]">
              <label
                htmlFor="password"
                className="text-[#D1D5DB] text-sm font-semibold "
              >
                Password
              </label>
              <div className="w-full relative mt-[0.78rem]">
                <input
                  type={isPasswordVisible ? "text" : "password"}
                  name="password"
                  id="password"
                  placeholder="Enter your password"
                  className="w-full h-[3rem] outline-none border rounded-[0.375rem] border-[#16213E] bg-[#050508] text-white placeholder:text-[#6B7280] text-sm pl-12"
                />
                <Image
                  src="/svgs/lock.svg"
                  alt="email"
                  width={20}
                  height={20}
                  className="absolute top-[1rem] left-[1rem]"
                />
                <Image
                  src="/svgs/eye.svg"
                  alt="email"
                  width={20}
                  height={20}
                  className="absolute top-[1rem] right-[1rem] cursor-pointer"
                  onClick={() => setIsPasswordVisible(!isPasswordVisible)}
                />
              </div>
            </div>

            <div className="mt-[1rem] w-full flex justify-between items-center">
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="remember"
                  className="appearance-none w-4 h-4 border-2 border-[#16213E] rounded-[0.375rem] bg-[#050508] checked:bg-[#00D4FF] checked:border-[#00D4FF] cursor-pointer"
                />
                <label
                  htmlFor="remember"
                  className="text-[#D1D5DB] text-sm font-semibold ml-2 cursor-pointer"
                >
                  Remember
                </label>
              </div>

              <p className="text-[#00D4FF] text-sm cursor-pointer">
                Forgot password?
              </p>
            </div>

            <button className='w-full h-[3rem] mt-[1rem] rounded-[0.375rem] bg-gradient-to-br from-[#00D4FF] to-[#A855F7] text-[1.125rem] text-[#0D0F11] font-bold cursor-pointer flex justify-center items-center'>SIGN IN</button>

            <p className="mt-[1.5rem] text-[#9CA3AF] text-base text-center">Don't have an account?  <span className="font-semibold text-[#00D4FF]">Sign up here</span></p>
          </form>

          <p className="text-xs text-[#6B7280] mt-[2rem]">By signing in, you agree to our  <Link href="/" className="text-[#00D4FF]">Terms of Service </Link>and <Link href="/" className="text-[#00D4FF]">Privacy Policy</Link></p>
        </div>
      </div>
    </main>
  );
}

export default SignIn;
