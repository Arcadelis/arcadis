import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import * as freighter from '@stellar/freighter-api';

interface WalletContextType {
  isWalletConnected: boolean;
  publicKey: string | null;
  network: string | null;
  connectWallet: () => Promise<void>;
  disconnectWallet: () => void;
}

const WalletContext = createContext<WalletContextType>({
  isWalletConnected: false,
  publicKey: null,
  network: null,
  connectWallet: async () => {},
  disconnectWallet: () => {},
});

export const useWallet = () => useContext(WalletContext);

interface WalletProviderProps {
  children: ReactNode;
}

export const WalletProvider = ({ children }: WalletProviderProps) => {
  const [isWalletConnected, setIsWalletConnected] = useState(false);
  const [publicKey, setPublicKey] = useState<string | null>(null);
  const [network, setNetwork] = useState<string | null>(null);

  useEffect(() => {
    checkWalletConnection();
  }, []);

  const checkWalletConnection = async () => {
    try {
      const connected = await freighter.isConnected();
      if (connected) {
        const key = await freighter.getPublicKey();
        const currentNetwork = await freighter.getNetwork();
        setIsWalletConnected(true);
        setPublicKey(key);
        setNetwork(currentNetwork);
      } else {
        setPublicKey(null);
      }
    } catch (error) {
      console.error('Error checking wallet connection:', error);
    }
  };

  const connectWallet = async () => {
    try {
      const key = await freighter.getPublicKey();
      const currentNetwork = await freighter.getNetwork();
      setIsWalletConnected(true);
      setPublicKey(key);
      setNetwork(currentNetwork);
    } catch (error) {
      console.error('Error connecting wallet:', error);
      throw error;
    }
  };

  const disconnectWallet = () => {
    setIsWalletConnected(false);
    setPublicKey(null);
    setNetwork(null);
  };

  const handleNetworkChange = (network: string) => {
    setNetwork(network);
  };

  return (
    <WalletContext.Provider
      value={{
        isWalletConnected,
        publicKey,
        network,
        connectWallet,
        disconnectWallet,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
};