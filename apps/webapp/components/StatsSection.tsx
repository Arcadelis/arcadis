import React from "react";
import { Trophy, Users, Target } from "lucide-react";
import { motion } from "framer-motion";

interface StatItem {
  icon: React.ComponentType<{ className?: string }>;
  value: string;
  label: string;
  color: {
    primary: string;
    secondary: string;
    glow: string;
  };
}

const statsData: StatItem[] = [
  {
    icon: Trophy,
    value: "500+",
    label: "INDIE GAMES",
    color: {
      primary: "text-cyan-400",
      secondary: "bg-cyan-500/20",
      glow: "border-cyan-500/30 shadow-cyan-500/20",
    },
  },
  {
    icon: Users,
    value: "50K+",
    label: "GAMERS",
    color: {
      primary: "text-purple-400",
      secondary: "bg-purple-500/20",
      glow: "border-purple-500/30 shadow-purple-500/20",
    },
  },
  {
    icon: Target,
    value: "200+",
    label: "DEVELOPERS",
    color: {
      primary: "text-green-400",
      secondary: "bg-green-500/20",
      glow: "border-green-500/30 shadow-green-500/20",
    },
  },
];

const StatsSection = () => {
  return (
    <section className="bg-[#050508] backdrop-blur-sm py-20 px-4 w-full relative z-10">
      <div className=" ">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-12 md:gap-16">
          {statsData.map((stat, index) => (
            <motion.div
              key={stat.label}
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.6, delay: index * 0.1 }}
              viewport={{ once: true }}
              className="text-center group"
            >
              {/* Icon Container with Enhanced Glow */}
              <motion.div
                whileHover={{ scale: 1.05 }}
                className={`inline-flex items-center justify-center w-20   mb-6  ${stat.color.glow}  backdrop-blur-sm`}
              >
                <stat.icon
                  className={`w-10 h-10 ${stat.color.primary} drop-shadow-lg`}
                />
              </motion.div>

              {/* Metric with Gradient Text */}
              <div
                className={`text-5xl md:text-6xl font-bold mb-3 ${stat.color.primary} drop-shadow-lg`}
              >
                {stat.value}
              </div>

              {/* Label */}
              <div className="text-gray-300 text-sm font-semibold tracking-widest uppercase">
                {stat.label}
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default StatsSection;
