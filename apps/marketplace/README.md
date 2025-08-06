# 🎮 Gaming Marketplace

A modern Next.js marketplace application for trading gaming assets, built with TypeScript, Tailwind CSS, and Bun.

## 🚀 Getting Started

### Prerequisites

- [Bun](https://bun.sh/) (recommended) or Node.js 18+
- Git

### Installation

1. Navigate to the marketplace directory:
   ```bash
   cd apps/marketplace
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Start the development server:
   ```bash
   bun run dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) in your browser.

## 🛠️ Tech Stack

- **Framework**: Next.js 15 (App Router)
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **Package Manager**: Bun
- **Linting**: ESLint

## 📁 Project Structure

```
apps/marketplace/
├── src/
│   └── app/
│       ├── layout.tsx      # Root layout
│       ├── page.tsx        # Home page
│       └── globals.css     # Global styles
├── public/                 # Static assets
├── package.json           # Dependencies and scripts
└── README.md             # This file
```

## 🎯 Features

- **Modern UI**: Beautiful gradient design with glassmorphism effects
- **Responsive**: Mobile-first design that works on all devices
- **Fast**: Built with Next.js 15 and optimized for performance
- **Type Safe**: Full TypeScript support for better development experience

## 🚀 Available Scripts

- `bun run dev` - Start development server
- `bun run build` - Build for production
- `bun run start` - Start production server
- `bun run lint` - Run ESLint

## 🎨 Customization

The marketplace is designed to be easily customizable:

- **Colors**: Modify the gradient in `src/app/page.tsx`
- **Content**: Update the marketplace sections and statistics
- **Styling**: Use Tailwind CSS classes for rapid styling

## 🔗 Integration

This marketplace is part of the larger Arcadis gaming ecosystem and can be integrated with:

- Smart contracts for asset trading
- User authentication systems
- Game integration APIs
- NFT marketplace functionality

## 📝 License

This project is part of the Arcadis ecosystem.
