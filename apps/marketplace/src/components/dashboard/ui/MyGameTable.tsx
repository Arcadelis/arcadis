"use client"

import Image from "next/image";
import React from "react";

interface MyGameTableProps {
  gameTitle: string;
  price: number;
  status: "published" | "draft";
  downloads: number;
  revenue: number;
  rating: number;
  updated: string;
}

function MyGameTable({ games }: { games: MyGameTableProps[] }) {
  return (
    <div className="w-full bg-[#121417] border border-[#16213E] rounded-[0.5rem] p-[1.6rem]">
      <p className="font-bold text-[1.5rem]">MY GAMES</p>

      <table className="w-full mt-[2.25rem]">
        <thead className="mb-[4rem]">
          <tr className="w-full flex text-sm font-bold text-[#00D4FF]">
            <th className="w-[12.5%] text-left">GAME TITLE</th>
            <th className="w-[12.5%] text-left">PRICE</th>
            <th className="w-[12.5%] text-left">STATUS</th>
            <th className="w-[12.5%] text-left">DOWNLOADS</th>
            <th className="w-[12.5%] text-left">REVENUE</th>
            <th className="w-[12.5%] text-left">RATING</th>
            <th className="w-[12.5%] text-left">UPDATED</th>
            <th className="w-[12.5%] text-left">ACTIONS</th>
          </tr>
        </thead>
        <tbody>
          {games.map((game, index) => (
            <tr
              key={index}
              className="w-full flex text-sm text-white mt-4 h-[4.3rem] border-t border-t-[#16213E] items-center"
            >
              <td className="w-[12.5%] font-bold">{game.gameTitle}</td>
              <td className="w-[12.5%] font-semibold text-[#00FF88]">
                ${game.price.toFixed(2)}
              </td>
              <td className="w-[12.5%]">
                <span
                  className={`px-2 py-1 rounded-full text-xs font-semibold ${
                    game.status === "published"
                      ? "bg-[#00FF8833] text-[#00FF88] border border-[#00FF884D]"
                      : "bg-[#4B556333] text-[#9CA3AF]"
                  }`}
                >
                  {game.status}
                </span>
              </td>
              <td className="w-[12.5%] font-semibold">
                {game.downloads.toLocaleString()}
              </td>
              <td className="w-[12.5%] font-bold text-[#00D4FF]">
                $
                {game.revenue.toLocaleString(undefined, {
                  minimumFractionDigits: 2,
                  maximumFractionDigits: 2,
                })}
              </td>
              <td className="w-[12.5%] font-bold text-[#FACC15] flex items-center">
                {game.rating > 0 ? (
                  <>
                    <Image
                      src="/svgs/star.svg"
                      alt="rating"
                      width={13.33}
                      height={12.68}
                    />{" "}
                    {game.rating.toFixed(1)}
                  </>
                ) : (
                  <span className="font-normal text-[#6B7280]">No ratings</span>
                )}
              </td>
              <td className="w-[12.5%] text-[#9CA3AF]">{game.updated}</td>
              <td className="w-[12.5%]">
                <div className="flex gap-3.5">
                  <button
                    className="w-[2.625rem] h-[2.25rem] rounded-[0.375rem] bg-[#0D0F11] border border-[#16213E] flex justify-center items-center cursor-pointer"
                    onClick={() => console.log("View", game.gameTitle)}
                  >
                    <Image
                      src="/svgs/view.svg"
                      alt="view"
                      width={16}
                      height={16}
                    />
                  </button>
                  <button
                    className="w-[2.625rem] h-[2.25rem] rounded-[0.375rem] bg-[#0D0F11] border border-[#16213E] flex justify-center items-center cursor-pointer"
                    onClick={() =>
                      game.status !== "published" &&
                      console.log("Edit", game.gameTitle)
                    }
                  >
                    <Image
                      src="/svgs/edit.svg"
                      alt="view"
                      width={16}
                      height={16}
                    />
                  </button>
                  <button
                    className="w-[2.625rem] h-[2.25rem] rounded-[0.375rem] bg-[#0D0F11] border border-[#16213E] flex justify-center items-center cursor-pointer"
                    onClick={() => console.log("Delete", game.gameTitle)}
                  >
                    <Image
                      src="/svgs/delete.svg"
                      alt="view"
                      width={16}
                      height={16}
                    />
                  </button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default MyGameTable;
