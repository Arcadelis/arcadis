# Arcadelis - Blockchain Gaming Platform

Arcadelis is a next-generation blockchain gaming platform built on Stellar, designed for game discovery, community engagement, awards, and developer tools. The platform integrates Stellar wallets and leverages Next.js 15, Tailwind CSS, shadcn/ui, Supabase, and Bun.

## 🚀 Features

- **Game Discovery**: Browse and explore blockchain games by category
- **Community Engagement**: Connect with other gamers and developers
- **Awards System**: Recognition for outstanding games and achievements
- **Wallet Integration**: Seamless Stellar wallet connectivity
- **Developer Tools**: Resources for game developers

## 🏗 Tech Stack

- **Frontend:** Next.js 14 + (App Router & Pages Router)
- **Backend:** Supabase (Database & Auth)
- **Styling:** Tailwind CSS & shadcn/ui
- **Package Management:** Bun
- **Smart Contracts:** Stellar integration
- **Security:** Secure wallet connections, protected routes, input validation, rate limiting

---

## 📂 Project Structure

```
Arcadelis/
├── .husky/                 # Pre-commit hooks
├── apps/
│   ├── admin/              # Admin panel (TBD)
│   ├── contracts/          # Smart contracts
│   ├── webapp/             # Main Next.js application
│   │   ├── .next/          # Build artifacts
│   │   ├── app/            # Next.js App Router pages
│   │   ├── hooks/          # React hooks
│   │   ├── lib/            # Utility functions and helpers
│   │   ├── public/         # Static assets
│   │   ├── .env            # Environment variables
│   │   ├── .env.sample     # Example environment file
│   │   ├── components.json # shadcn/ui components
│   │   ├── eslint.config.mjs  # Linter configuration
│   │   ├── next.config.ts  # Next.js configuration
│   │   ├── package.json    # Dependencies
│   │   ├── tailwind.config.ts # Tailwind CSS configuration
│   │   ├── tsconfig.json   # TypeScript configuration
│   │   └── README.md       # Webapp documentation
├── docs/                   # Documentation
├── node_modules/           # Dependencies
├── services/               # Backend services
│   ├── supabase/           # API services
│   └── README.md           # Services documentation
├── .gitignore              # Git ignored files
├── bun.lock                # Bun package lock file
├── commitlint.config.js    # Commit message rules
├── README.md               # Project documentation
└── package.json            # Root package dependencies
```

---

## 🏃 Getting Started

### Prerequisites

Ensure you have the following installed:
- [Node.js (v18 or higher)](https://nodejs.org/)
- [Bun](https://bun.sh/)
- [Git](https://git-scm.com/)

### Clone and Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/arcadelis.git
   cd arcadelis
   ```

2. Install dependencies:
   ```bash
   bun install
   ```

3. Start the development server:
   ```bash
   bun dev
   ```

4. Rename `.env.sample` to `.env.local` and update the following:
   ```
   NEXT_PUBLIC_SUPABASE_URL=[INSERT SUPABASE PROJECT URL]
   NEXT_PUBLIC_SUPABASE_ANON_KEY=[INSERT SUPABASE PROJECT API ANON KEY]
   ```
   You can find these values in [Supabase API settings](https://app.supabase.com/project/_/settings/api).

---

## 🏗 Architecture Overview

### Frontend
- Uses **Next.js 14** with the **App Router**
- Modular structure with reusable components
- Optimized with **shadcn/ui** and **Tailwind CSS**

### Backend
- Built with **Supabase** for database and authentication
- API services managed under `services/`

### Key Features

#### 🎮 Game Discovery
- Browse games by category
- Search functionality
- Detailed game pages

#### 🏆 Awards System
- Hall of Fame
- Monthly Awards
- Community Nominations

#### 👤 User Profiles
- Wallet integration
- Game history
- Achievements

---

## 🔒 Security & Best Practices
- **Secure Wallet Connections**: Ensures safe user authentication
- **Protected Routes**: Prevents unauthorized access
- **Input Validation**: Reduces security risks
- **Rate Limiting**: Protects against abuse

---

## 📜 License
This project is open-source and available under the [MIT License](LICENSE).

## 🚀 Contributing
We welcome contributions! Feel free to submit pull requests or open issues.

---

Made with ❤️ by the Arcadelis Team

